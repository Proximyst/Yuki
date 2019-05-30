use super::{mutmemory::MutableMemory, prelude::*};
use getset::Getters;
use std::{
    borrow::{Borrow, BorrowMut},
    convert::{AsMut, AsRef},
    mem,
    ops::Deref,
    ptr,
};
use winapi::{
    shared::minwindef,
    um::{libloaderapi, processthreadsapi, winnt},
};

pub type ThisCallFn<T> = unsafe extern "thiscall" fn(thisptr: *const usize) -> T;

static mut CURRENT_PROCESS: Option<GameProcess> = None;

#[derive(Getters, Clone, Copy, Debug)]
#[get = "pub"]
pub struct GameProcess {
    handle: winnt::HANDLE,
    pid: u32,
    base: minwindef::HMODULE,
    mut_mem: MutableMemory,
}

#[derive(Getters, Clone, Copy, Debug)]
#[get = "pub"]
pub struct Module {
    handle: minwindef::HMODULE,
    mut_mem: MutableMemory,
    parent: *mut GameProcess,
}

#[derive(Getters, Clone, Copy, Debug)]
#[get = "pub"]
pub struct Interface {
    handle: *const usize,
    mut_mem: MutableMemory,
    parent: *mut Module,
    methods: usize,
}

impl GameProcess {
    pub fn current_process() -> Self {
        unsafe {
            if let Some(g) = CURRENT_PROCESS {
                return g;
            }
        }

        let current = Self::new(unsafe { processthreadsapi::GetCurrentProcess() });
        unsafe {
            CURRENT_PROCESS = Some(current);
        }

        current
    }

    pub fn new(handle: winnt::HANDLE) -> Self {
        let pid = unsafe { processthreadsapi::GetProcessId(handle) };
        let base = unsafe { libloaderapi::GetModuleHandleA(ptr::null()) };
        GameProcess {
            handle,
            pid,
            base,
            mut_mem: MutableMemory::new(base as _),
        }
    }

    pub fn read<T>(&self, offset: isize) -> &T {
        self.mut_mem.read(offset)
    }

    pub fn read_mut<T>(&self, offset: isize) -> &mut T {
        self.mut_mem.read_mut(offset)
    }

    pub fn write<T>(&self, offset: isize, value: T) {
        self.mut_mem.write(offset, value)
    }

    pub fn get_module(&mut self, module_name: &str) -> Result<Module> {
        let module_name_null = &format!("{}\0", module_name);

        let module =
            unsafe { libloaderapi::GetModuleHandleA(module_name_null.as_ptr() as *const _) };
        if module.is_null() {
            return Err(ProcessErrorKind::UnknownModule(module_name.into()).into());
        }

        let module = Module {
            handle: module,
            mut_mem: MutableMemory::new(module as _),
            parent: self as *mut _,
        };

        Ok(module)
    }
}

impl Module {
    pub fn read<T>(&self, offset: isize) -> &T {
        self.mut_mem.read(offset)
    }

    pub fn read_mut<T>(&self, offset: isize) -> &mut T {
        self.mut_mem.read_mut(offset)
    }

    pub fn write<T>(&self, offset: isize, value: T) {
        self.mut_mem.write(offset, value)
    }

    pub fn get_export(&mut self, export_name: &str) -> Result<minwindef::FARPROC> {
        let export_name_null = &format!("{}\0", export_name);
        let farproc = unsafe {
            libloaderapi::GetProcAddress(self.handle, export_name_null.as_ptr() as *const _)
        };

        if farproc.is_null() {
            return Err(ProcessErrorKind::UnknownExport(export_name.into()).into());
        }

        Ok(farproc)
    }

    pub fn create_interface(&mut self, interface_name: &str) -> Result<Interface> {
        let interface_name_null = &format!("{}\0", interface_name);
        let create_interface = self.get_export("CreateInterface")?;
        let create_interface = unsafe {
            mem::transmute::<
                _,
                unsafe extern "C" fn(
                    name: *const libc::c_char,
                    return_code: *const libc::c_int,
                ) -> *const libc::c_void,
            >(create_interface)
        };

        let interface = unsafe {
            create_interface(interface_name_null.as_ptr() as *const _, ptr::null_mut())
                as *const usize
        };

        if interface.is_null() {
            return Err(ProcessErrorKind::UnknownInterface(interface_name.into()).into());
        }

        Ok(Interface::new(interface, self as *mut Module))
    }

    pub unsafe fn pattern_scan(&mut self, bytes: &[Option<u8>]) -> Option<*mut u8> {
        let dos_header = self.handle as winnt::PIMAGE_DOS_HEADER;
        let nt_headers = (self.handle as *const u8).offset((*dos_header).e_lfanew as _)
            as winnt::PIMAGE_NT_HEADERS;
        let image_size = (*nt_headers).OptionalHeader.SizeOfImage;

        let scan_bytes = self.handle as *mut u8;

        let bytes_len = bytes.len();

        'a: for i in 0..(image_size as usize - bytes_len) {
            for j in 0..bytes_len {
                let byte = match bytes[j] {
                    None => continue,
                    Some(s) => s,
                };

                if *scan_bytes.offset((i + j) as isize) != byte {
                    continue 'a;
                }
            }

            return Some(scan_bytes.offset(i as isize));
        }

        None
    }
}

impl Interface {
    pub fn new(ptr: *const usize, parent: *mut Module) -> Self {
        let vtable = unsafe { *ptr as *const usize };
        let methods = {
            let mut count = 0;
            while !unsafe { vtable.offset(count).read() as *const usize }.is_null() {
                count += 1;
            }
            count as usize
        };

        Interface {
            handle: ptr,
            mut_mem: MutableMemory::new(ptr as _),
            parent,
            methods,
        }
    }

    pub fn read<T>(&self, offset: isize) -> &T {
        self.mut_mem.read(offset)
    }

    pub fn read_mut<T>(&self, offset: isize) -> &mut T {
        self.mut_mem.read_mut(offset)
    }

    pub fn write<T>(&self, offset: isize, value: T) {
        self.mut_mem.write(offset, value)
    }

    pub fn vtable(&self) -> *const usize {
        unsafe { *(self.handle) as *const usize }
    }

    pub fn nth(&self, index: isize) -> Result<*const usize> {
        if index as usize > self.methods {
            return Err(InterfaceErrorKind::InvalidVFuncIndex(index).into());
        }

        Ok(unsafe { self.vtable().offset(index).read() as *const usize })
    }
}

#[derive(Clone, Copy, Debug)]
pub struct VmtInterface {
    inner: Interface,
    own_table: *const usize,
    original_table: *const usize,
}

impl VmtInterface {
    pub fn new(inner: Interface) -> VmtInterface {
        let vtable = inner.vtable();
        let methods = *inner.methods();

        let own_table = Vec::with_capacity(methods).as_mut_ptr();
        unsafe {
            ptr::copy_nonoverlapping(vtable, own_table, methods);
        }

        VmtInterface {
            inner,
            own_table,
            original_table: vtable,
        }
    }

    pub fn apply_vmt(&self) {
        unsafe {
            ptr::write((*self.inner.handle()) as *mut _, self.own_table);
        }
    }

    pub fn release_vmt(&self) {
        unsafe {
            ptr::write((*self.inner.handle()) as *mut _, self.original_table);
        }
    }

    pub fn hook_vfunc(&self, func: *const usize, index: isize) -> Result<()> {
        if index as usize > *self.inner.methods() {
            return Err(InterfaceErrorKind::InvalidVFuncIndex(index).into());
        }
        unsafe {
            ptr::write(self.own_table.offset(index) as *mut _, func);
        }
        Ok(())
    }
}

impl AsRef<Interface> for VmtInterface {
    fn as_ref(&self) -> &Interface {
        &self.inner
    }
}

impl AsMut<Interface> for VmtInterface {
    fn as_mut(&mut self) -> &mut Interface {
        &mut self.inner
    }
}

impl Deref for VmtInterface {
    type Target = Interface;

    fn deref(&self) -> &Interface {
        self.as_ref()
    }
}

impl Borrow<Interface> for VmtInterface {
    fn borrow(&self) -> &Interface {
        self.as_ref()
    }
}

impl BorrowMut<Interface> for VmtInterface {
    fn borrow_mut(&mut self) -> &mut Interface {
        self.as_mut()
    }
}

unsafe impl Sync for VmtInterface {}
unsafe impl Send for VmtInterface {}
