use super::super::prelude::*;
use std::{cell::Cell, mem::transmute, ptr};

thread_local! {
    static CLANTAG_NAME_FUNC: Cell<*const libc::c_void> = Cell::new(ptr::null());
}
type ClantagNameFuncType = unsafe extern "fastcall" fn(*const u8, *const u8) -> i32;

#[repr(isize)]
pub enum EngineVTableIndicies {
    ScreenSize = 5,
    LocalPlayer = 12,
    LastTimeStamp = 14,
    MaxClients = 20,
    IsInGame = 26,
    IsConnected = 27,
    ClientCommand = 108,
    UnrestrictedClientCommand = 114,
}

#[derive(Debug, Copy, Clone)]
pub struct EngineInterface {
    inner: Interface,
}

impl EngineInterface {
    pub const fn new(inner: Interface) -> Self {
        EngineInterface { inner }
    }

    pub fn get_screen_size(&self) -> Result<(i32, i32)> {
        type Func = unsafe extern "thiscall" fn(*const usize, *mut i32, *mut i32) -> i32;
        let mut width = 0;
        let mut height = 0;
        unsafe {
            let _ =
                transmute::<_, Func>(self.inner.nth(EngineVTableIndicies::ScreenSize as isize)?)(
                    *self.inner.handle(),
                    &mut width as *mut _,
                    &mut height as *mut _,
                );
        }

        Ok((width, height))
    }

    pub fn get_local_player(&self) -> Result<i32> {
        type Func = unsafe extern "thiscall" fn(*const usize) -> i32;

        Ok(unsafe {
            transmute::<_, Func>(self.inner.nth(EngineVTableIndicies::LocalPlayer as isize)?)(
                *self.inner.handle(),
            )
        })
    }

    pub fn get_last_timestamp(&self) -> Result<f32> {
        type Func = unsafe extern "thiscall" fn(*const usize) -> f32;

        Ok(unsafe {
            transmute::<_, Func>(
                self.inner
                    .nth(EngineVTableIndicies::LastTimeStamp as isize)?,
            )(*self.inner.handle())
        })
    }

    pub fn get_max_clients(&self) -> Result<i32> {
        type Func = unsafe extern "thiscall" fn(*const usize) -> i32;

        Ok(unsafe {
            transmute::<_, Func>(self.inner.nth(EngineVTableIndicies::MaxClients as isize)?)(
                *self.inner.handle(),
            )
        })
    }

    pub fn is_in_game(&self) -> Result<bool> {
        type Func = unsafe extern "thiscall" fn(*const usize) -> i32;
        Ok(unsafe {
            transmute::<_, Func>(self.inner.nth(EngineVTableIndicies::IsInGame as isize)?)(
                *self.inner.handle(),
            )
        } != (false as i32))
    }

    pub fn is_connected(&self) -> Result<bool> {
        type Func = unsafe extern "thiscall" fn(*const usize) -> i32;
        Ok(unsafe {
            transmute::<_, Func>(self.inner.nth(EngineVTableIndicies::IsConnected as isize)?)(
                *self.inner.handle(),
            )
        } != (false as i32))
    }

    pub fn run_client_cmd(&self, command: &str) -> Result<()> {
        type Func = unsafe extern "thiscall" fn(*const usize, *const libc::c_char) -> ();
        let command = format!("{}\0", command);
        unsafe {
            transmute::<_, Func>(
                self.inner
                    .nth(EngineVTableIndicies::ClientCommand as isize)?,
            )(*self.inner.handle(), command.as_ptr() as *const _);
        }

        Ok(())
    }

    pub fn run_client_cmd_unrestricted(&self, command: &str) -> Result<()> {
        type Func = unsafe extern "thiscall" fn(*const usize, *const libc::c_char, u8) -> ();
        let command = format!("{}\0", command);
        unsafe {
            transmute::<_, Func>(
                self.inner
                    .nth(EngineVTableIndicies::UnrestrictedClientCommand as isize)?,
            )(*self.inner.handle(), command.as_ptr() as *const _, 1);
        }

        Ok(())
    }

    pub fn set_clantag_and_name(&self, sz_clantag: &str, sz_name: &str) -> Result<i32> {
        CLANTAG_NAME_FUNC.with(move |r#static| {
            let fn_ptr = r#static.get();
            let func: ClantagNameFuncType =
            if fn_ptr.is_null() {
                let module = unsafe { &mut **self.inner.parent() };
                let function_address = unsafe {
                    module.pattern_scan(&[
                        Some(0x53),
                        Some(0x56),
                        Some(0x57),
                        Some(0x8B),
                        Some(0xDA),
                        Some(0x8B),
                        Some(0xF9),
                        Some(0xFF),
                        Some(0x15),
                    ])
                }
                .failure()?;
                let function = unsafe {transmute::<_, ClantagNameFuncType>(function_address)};
                r#static.set(function_address as *const _);
                function
            } else {
                unsafe { transmute::<_, ClantagNameFuncType>(fn_ptr) }
            };

            unsafe {
                Ok(func(sz_clantag.as_ptr(), sz_name.as_ptr()))
            }
        })
    }
}
