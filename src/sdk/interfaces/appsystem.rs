use std::{ffi::c_void, mem::transmute};

use getset::Getters;

use super::super::prelude::*;

#[derive(Debug, Copy, Clone, Getters)]
#[get = "pub"]
pub struct AppSystemInterface {
    inner: Interface,
}

#[repr(isize)]
pub enum AppSystemVTableIndicies {
    Connect = 0,
    Disconnect = 1,
    QueryInterface = 2,
    Init = 3,
    Shutdown = 4,
    GetDependencies = 5,
    GetTier = 6,
    Reconnect = 7,
    UnkFunc = 8,
}

impl AppSystemInterface {
    pub const fn new(inner: Interface) -> Self {
        AppSystemInterface { inner }
    }

    pub fn connect(
        &self,
        factory: unsafe extern "thiscall" fn(*const libc::c_char, *mut i32) -> *const c_void,
    ) -> Result<bool> {
        type Func = unsafe extern "thiscall" fn(
            *const usize,
            unsafe extern "thiscall" fn(*const libc::c_char, *mut i32) -> *const c_void,
        ) -> bool;

        Ok(unsafe {
            transmute::<_, Func>(self.inner.nth(AppSystemVTableIndicies::Connect as isize)?)(
                *self.inner.handle(),
                factory,
            )
        })
    }
    pub fn disconnect(&self) -> Result<()> {
        type Func = unsafe extern "thiscall" fn(*const usize);

        Ok(unsafe {
            transmute::<_, Func>(
                self.inner
                    .nth(AppSystemVTableIndicies::Disconnect as isize)?,
            )(*self.inner.handle())
        })
    }

    pub fn query_interface(&self, interface_name: *const libc::c_char) -> Result<()> {
        type Func = unsafe extern "thiscall" fn(*const usize, *const libc::c_char);

        Ok(unsafe {
            transmute::<_, Func>(
                self.inner
                    .nth(AppSystemVTableIndicies::QueryInterface as isize)?,
            )(*self.inner.handle(), interface_name)
        })
    }

    pub fn init(&self) -> Result<i32 /* InitReturnVal_t */> {
        type Func = unsafe extern "thiscall" fn(*const usize) -> i32;

        Ok(unsafe {
            transmute::<_, Func>(self.inner.nth(AppSystemVTableIndicies::Init as isize)?)(
                *self.inner.handle(),
            )
        })
    }
    pub fn shutdown(&self) -> Result<()> {
        type Func = unsafe extern "thiscall" fn(*const usize);

        Ok(unsafe {
            transmute::<_, Func>(self.inner.nth(AppSystemVTableIndicies::Shutdown as isize)?)(
                *self.inner.handle(),
            )
        })
    }

    pub fn get_dependencies(&self) -> Result<*const c_void /* AppSystemInfo_t */> {
        type Func = unsafe extern "thiscall" fn(*const usize) -> *const c_void;

        Ok(unsafe {
            transmute::<_, Func>(
                self.inner
                    .nth(AppSystemVTableIndicies::GetDependencies as isize)?,
            )(*self.inner.handle())
        })
    }

    pub fn get_tier(&self) -> Result<i32 /* AppSystemTier_t */> {
        type Func = unsafe extern "thiscall" fn(*const usize) -> i32;

        Ok(unsafe {
            transmute::<_, Func>(self.inner.nth(AppSystemVTableIndicies::GetTier as isize)?)(
                *self.inner.handle(),
            )
        })
    }

    pub fn reconnect(
        &self,
        factory: unsafe extern "thiscall" fn(*const libc::c_char, *mut i32) -> *const c_void,
        interface_name: *const libc::c_char,
    ) -> Result<()> {
        type Func = unsafe extern "thiscall" fn(
            *const usize,
            unsafe extern "thiscall" fn(*const libc::c_char, *mut i32) -> *const c_void,
            *const libc::c_char,
        );

        Ok(unsafe {
            transmute::<_, Func>(
                self.inner
                    .nth(AppSystemVTableIndicies::Reconnect as isize)?,
            )(*self.inner.handle(), factory, interface_name)
        })
    }

    pub fn unk_func(&self) -> Result<()> {
        type Func = unsafe extern "thiscall" fn(*const usize);

        Ok(unsafe {
            transmute::<_, Func>(self.inner.nth(AppSystemVTableIndicies::UnkFunc as isize)?)(
                *self.inner.handle(),
            )
        })
    }
}
