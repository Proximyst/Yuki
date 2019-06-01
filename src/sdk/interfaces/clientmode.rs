use std::{ffi::c_void, mem::transmute};

use getset::Getters;

use super::super::prelude::*;

#[derive(Debug, Copy, Clone, Getters)]
#[get = "pub"]
pub struct ClientModeInterface {
    inner: Interface,
}

#[repr(isize)]
pub enum ClientModeVTableIndicies {
    ShouldDrawEntity = 14,
}

impl ClientModeInterface {
    pub const fn new(inner: Interface) -> Self {
        ClientModeInterface { inner }
    }

    pub fn should_draw_entity(
        &self,
        client_entity: &super::super::defs::cliententity::ClientEntity,
    ) -> Result<bool> {
        type Func = unsafe extern "thiscall" fn(*const usize, *const c_void) -> bool;

        Ok(unsafe {
            transmute::<_, Func>(
                self.inner
                    .nth(ClientModeVTableIndicies::ShouldDrawEntity as isize)?,
            )(*self.inner.handle(), *client_entity.ptr())
        })
    }
}
