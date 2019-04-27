use super::prelude::*;
use getset::Getters;
use std::{mem, ptr};
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
    base_addr: usize,
}

#[derive(Getters, Clone, Copy, Debug)]
#[get = "pub"]
pub struct Module {
    handle: minwindef::HMODULE,
    base_addr: usize,
    parent: *mut GameProcess,
}

#[derive(Getters, Clone, Copy, Debug)]
#[get = "pub"]
#[repr(transparent)]
pub struct Interface {
    handle: *const usize,
}

impl GameProcess {
    fn fix_address(&self, address: usize) -> usize {
        self.base_addr + address
    }

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
            base_addr: unsafe { *(base as *mut libc::c_void as *mut u32) } as usize,
        }
    }

    pub unsafe fn read<T>(&self, address: usize) -> &T {
        &*(self.fix_address(address) as *const T)
    }

    pub unsafe fn read_mut<T>(&mut self, address: usize) -> &mut T {
        &mut *(self.fix_address(address) as *mut T)
    }

    pub unsafe fn write<T>(&mut self, address: usize, value: T) {
        *(self.fix_address(address) as *mut T) = value;
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
            base_addr: unsafe { *(module as *mut libc::c_void as *mut u32) } as usize,
            parent: self as *mut _,
        };

        Ok(module)
    }
}

impl Module {
    fn fix_offset(&self, offset: usize) -> usize {
        (self.base_addr as usize) + offset
    }

    pub unsafe fn read<T>(&self, offset: usize) -> &T {
        &*(self.fix_offset(offset) as *const T)
    }

    pub unsafe fn read_mut<T>(&self, offset: usize) -> &mut T {
        &mut *(self.fix_offset(offset) as *mut T)
    }

    pub unsafe fn write<T>(&mut self, offset: usize, value: T) {
        *(self.fix_offset(offset) as *mut T) = value;
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

        Ok(Interface::new(interface))
    }

    pub unsafe fn pattern_scan(&mut self, bytes: &[Option<u8>]) -> Option<*mut u8> {
        let dos_header = self.handle as winnt::PIMAGE_DOS_HEADER;
        let nt_headers = (self.handle as *const u8).offset((*dos_header).e_lfanew as _)
            as winnt::PIMAGE_NT_HEADERS;
        let image_size = (*nt_headers).OptionalHeader.SizeOfImage;

        let scan_bytes = self.handle as *mut u8;

        let bytes_len = bytes.len();

        for i in 0..(image_size as usize - bytes_len) {
            let mut found = true;

            for j in 0..bytes_len {
                let byte = match bytes[j] {
                    None => continue,
                    Some(s) => s,
                };

                if *scan_bytes.offset((i + j) as isize) != byte {
                    found = false;
                    break;
                }
            }

            if found {
                return Some(scan_bytes.offset(i as isize));
            }
        }

        None
    }
}

impl Interface {
    pub const fn new(ptr: *const usize) -> Self {
        Interface { handle: ptr }
    }

    pub fn vtable(&self) -> *const usize {
        unsafe { *(self.handle) as *const usize }
    }

    pub fn nth(&self, index: isize) -> Result<*const usize> {
        let func = unsafe { self.vtable().offset(index).read() as *const usize };

        if func.is_null() {
            return Err(InterfaceErrorKind::InvalidVFuncIndex(index).into());
        }

        Ok(func)
    }
}
