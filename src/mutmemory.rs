use derive_more::Display;
use std::ffi::c_void;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Display)]
#[repr(transparent)]
#[display(fmt = "MutableMemory({:?})", base)]
pub struct MutableMemory {
    base: *mut c_void,
}

impl MutableMemory {
    pub const fn new(base: *mut c_void) -> Self {
        MutableMemory { base }
    }

    pub fn read<T>(&self, offset: isize) -> &T {
        unsafe { &*(self.base.offset(offset) as *mut T as *const _) }
    }

    pub fn read_mut<T>(&self, offset: isize) -> &mut T {
        unsafe { &mut *(self.base.offset(offset) as *mut T) }
    }

    pub fn write<T>(&self, offset: isize, value: T) {
        unsafe {
            *(self.base.offset(offset) as *mut T) = value;
        }
    }
}

impl<T> Into<*const T> for MutableMemory {
    fn into(self) -> *const T {
        self.base as _
    }
}

impl<T> Into<*mut T> for MutableMemory {
    fn into(self) -> *mut T {
        self.base as _
    }
}

unsafe impl Send for MutableMemory {}
unsafe impl Sync for MutableMemory {}
