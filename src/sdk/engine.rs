use super::prelude::*;
use std::mem::transmute;

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
}
