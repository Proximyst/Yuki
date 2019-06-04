use std::{ffi::OsString, mem::transmute};

use getset::Getters;
use wio::wide::FromWide as _;

use super::super::prelude::*;

#[derive(Debug, Copy, Clone, Getters)]
#[get = "pub"]
pub struct LocalizeInterface {
    inner: Interface,
}

#[repr(isize)]
pub enum LocalizeVTableIndicies {
    Find = 11,
}

impl LocalizeInterface {
    pub const fn new(inner: Interface) -> Self {
        LocalizeInterface { inner }
    }

    pub fn find(&self, token_name: *const libc::c_char) -> Result<OsString> {
        type Func = unsafe extern "thiscall" fn(*const usize, *const libc::c_char) -> *const u16;

        let ptr = unsafe {
            transmute::<_, Func>(self.inner.nth(LocalizeVTableIndicies::Find as isize)?)(
                *self.inner.handle(),
                token_name,
            )
        };

        Ok(unsafe { OsString::from_wide_ptr_null(ptr) })
    }
}
