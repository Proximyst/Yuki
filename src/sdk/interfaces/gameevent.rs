//! <https://github.com/pmrowla/hl2sdk-csgo/blob/master/public/igameevents.h#L65>

use std::mem::transmute;

use getset::Getters;

use super::super::prelude::*;

#[derive(Debug, Copy, Clone, Getters)]
#[get = "pub"]
pub struct GameEventInterface {
    inner: Interface,
}

#[repr(isize)]
pub enum GameEventVTableIndicies {
    GetName = 1,
    IsReliable = 2,
    IsLocal = 3,
    IsEmpty = 4,
    GetBool = 5,
    GetInt = 6,
    GetUInt64 = 7,
    GetFloat = 8,
    GetString = 9,
    GetWString = 10,
    SetBool = 11,
    SetInt = 12,
    SetUInt64 = 13,
    SetFloat = 14,
    SetString = 15,
    SetWString = 16,
}

impl GameEventInterface {
    pub const fn new(inner: Interface) -> Self {
        GameEventInterface { inner }
    }

    pub fn get_name(&self) -> Result<*const libc::c_char> {
        type Func = unsafe extern "thiscall" fn(*const usize) -> *const libc::c_char;

        Ok(unsafe {
            transmute::<_, Func>(self.inner.nth(GameEventVTableIndicies::GetName as isize)?)(
                *self.inner.handle(),
            )
        })
    }

    pub fn is_reliable(&self) -> Result<bool> {
        type Func = unsafe extern "thiscall" fn(*const usize) -> bool;

        Ok(unsafe {
            transmute::<_, Func>(
                self.inner
                    .nth(GameEventVTableIndicies::IsReliable as isize)?,
            )(*self.inner.handle())
        })
    }

    pub fn is_local(&self) -> Result<bool> {
        type Func = unsafe extern "thiscall" fn(*const usize) -> bool;

        Ok(unsafe {
            transmute::<_, Func>(self.inner.nth(GameEventVTableIndicies::IsLocal as isize)?)(
                *self.inner.handle(),
            )
        })
    }

    pub fn is_empty(&self, key_name: *const libc::c_char) -> Result<bool> {
        type Func = unsafe extern "thiscall" fn(*const usize, *const libc::c_char) -> bool;

        Ok(unsafe {
            transmute::<_, Func>(self.inner.nth(GameEventVTableIndicies::IsEmpty as isize)?)(
                *self.inner.handle(),
                key_name,
            )
        })
    }

    pub fn get_bool(&self, key_name: *const libc::c_char, default_value: bool) -> Result<bool> {
        type Func = unsafe extern "thiscall" fn(*const usize, *const libc::c_char, bool) -> bool;

        Ok(unsafe {
            transmute::<_, Func>(self.inner.nth(GameEventVTableIndicies::GetBool as isize)?)(
                *self.inner.handle(),
                key_name,
                default_value,
            )
        })
    }

    pub fn get_int(&self, key_name: *const libc::c_char, default_value: i32) -> Result<i32> {
        type Func = unsafe extern "thiscall" fn(*const usize, *const libc::c_char, i32) -> i32;

        Ok(unsafe {
            transmute::<_, Func>(self.inner.nth(GameEventVTableIndicies::GetInt as isize)?)(
                *self.inner.handle(),
                key_name,
                default_value,
            )
        })
    }

    pub fn get_uint64(&self, key_name: *const libc::c_char, default_value: u64) -> Result<u64> {
        type Func = unsafe extern "thiscall" fn(*const usize, *const libc::c_char, u64) -> u64;

        Ok(unsafe {
            transmute::<_, Func>(
                self.inner
                    .nth(GameEventVTableIndicies::GetUInt64 as isize)?,
            )(*self.inner.handle(), key_name, default_value)
        })
    }

    pub fn get_float(&self, key_name: *const libc::c_char, default_value: f32) -> Result<f32> {
        type Func = unsafe extern "thiscall" fn(*const usize, *const libc::c_char, f32) -> f32;

        Ok(unsafe {
            transmute::<_, Func>(self.inner.nth(GameEventVTableIndicies::GetFloat as isize)?)(
                *self.inner.handle(),
                key_name,
                default_value,
            )
        })
    }

    pub fn get_string(
        &self,
        key_name: *const libc::c_char,
        default_value: *const libc::c_char,
    ) -> Result<*const libc::c_char> {
        type Func = unsafe extern "thiscall" fn(
            *const usize,
            *const libc::c_char,
            *const libc::c_char,
        ) -> *const libc::c_char;

        Ok(unsafe {
            transmute::<_, Func>(
                self.inner
                    .nth(GameEventVTableIndicies::GetString as isize)?,
            )(*self.inner.handle(), key_name, default_value)
        })
    }

    pub fn get_wide_string(
        &self,
        key_name: *const libc::c_char,
        default_value: *const u16,
    ) -> Result<*const u16> {
        type Func = unsafe extern "thiscall" fn(
            *const usize,
            *const libc::c_char,
            *const u16,
        ) -> *const u16;

        Ok(unsafe {
            transmute::<_, Func>(
                self.inner
                    .nth(GameEventVTableIndicies::GetWString as isize)?,
            )(*self.inner.handle(), key_name, default_value)
        })
    }

    pub fn set_bool(&self, key_name: *const libc::c_char, value: bool) -> Result<()> {
        type Func = unsafe extern "thiscall" fn(*const usize, *const libc::c_char, bool);

        unsafe {
            transmute::<_, Func>(self.inner.nth(GameEventVTableIndicies::SetBool as isize)?)(
                *self.inner.handle(),
                key_name,
                value,
            );
        }

        Ok(())
    }

    pub fn set_int(&self, key_name: *const libc::c_char, value: i32) -> Result<()> {
        type Func = unsafe extern "thiscall" fn(*const usize, *const libc::c_char, i32);

        unsafe {
            transmute::<_, Func>(self.inner.nth(GameEventVTableIndicies::SetInt as isize)?)(
                *self.inner.handle(),
                key_name,
                value,
            );
        }

        Ok(())
    }

    pub fn set_uint64(&self, key_name: *const libc::c_char, value: u64) -> Result<()> {
        type Func = unsafe extern "thiscall" fn(*const usize, *const libc::c_char, u64);

        unsafe {
            transmute::<_, Func>(
                self.inner
                    .nth(GameEventVTableIndicies::SetUInt64 as isize)?,
            )(*self.inner.handle(), key_name, value);
        }

        Ok(())
    }

    pub fn set_float(&self, key_name: *const libc::c_char, value: f32) -> Result<()> {
        type Func = unsafe extern "thiscall" fn(*const usize, *const libc::c_char, f32);

        unsafe {
            transmute::<_, Func>(self.inner.nth(GameEventVTableIndicies::SetFloat as isize)?)(
                *self.inner.handle(),
                key_name,
                value,
            );
        }

        Ok(())
    }
    pub fn set_string(
        &self,
        key_name: *const libc::c_char,
        value: *const libc::c_char,
    ) -> Result<()> {
        type Func =
        unsafe extern "thiscall" fn(*const usize, *const libc::c_char, *const libc::c_char);

        unsafe {
            transmute::<_, Func>(
                self.inner
                    .nth(GameEventVTableIndicies::SetString as isize)?,
            )(*self.inner.handle(), key_name, value);
        }

        Ok(())
    }

    pub fn set_wide_string(&self, key_name: *const libc::c_char, value: *const u16) -> Result<()> {
        type Func = unsafe extern "thiscall" fn(*const usize, *const libc::c_char, *const u16);

        unsafe {
            transmute::<_, Func>(
                self.inner
                    .nth(GameEventVTableIndicies::SetWString as isize)?,
            )(*self.inner.handle(), key_name, value);
        }

        Ok(())
    }
}
