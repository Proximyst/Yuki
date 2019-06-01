use std::{ffi::c_void, mem::transmute};

use getset::Getters;

use super::super::{defs::cliententity::ClientEntity, prelude::*};

#[derive(Debug, Copy, Clone, Getters)]
#[get = "pub"]
pub struct EntityListInterface {
    inner: Interface,
}

#[repr(isize)]
pub enum EntityListVTableIndicies {
    GetClientEntityIndex = 3,
    GetClientEntityHandle = 4,
}

impl EntityListInterface {
    pub const fn new(inner: Interface) -> Self {
        EntityListInterface { inner }
    }

    pub fn get_client_entity_by_id(&self, id: i32) -> Result<ClientEntity> {
        type Func = unsafe extern "thiscall" fn(*const usize, i32) -> *const c_void;

        Ok(ClientEntity::new(unsafe {
            transmute::<_, Func>(
                self.inner
                    .nth(EntityListVTableIndicies::GetClientEntityIndex as isize)?,
            )(*self.inner.handle(), id)
        }))
    }

    pub fn get_client_entity_by_handle(&self, handle: *const c_void) -> Result<ClientEntity> {
        type Func = unsafe extern "thiscall" fn(*const usize, *const c_void) -> *const c_void;

        Ok(ClientEntity::new(unsafe {
            transmute::<_, Func>(
                self.inner
                    .nth(EntityListVTableIndicies::GetClientEntityHandle as isize)?,
            )(*self.inner.handle(), handle)
        }))
    }
}
