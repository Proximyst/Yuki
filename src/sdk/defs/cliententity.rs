use std::{ffi::c_void, mem::transmute};

use getset::Getters;

use super::super::prelude::*;

#[derive(Getters, Copy, Clone)]
#[get = "pub"]
pub struct ClientEntity {
    ptr: *const c_void,
    entity_id_this: VTable,
}

#[repr(isize)]
pub enum ClientEntityVTableIndicies {
    GetEntityId = 10,
}

impl ClientEntity {
    pub fn new(ptr: *const c_void) -> Self {
        ClientEntity {
            ptr,
            entity_id_this: VTable::new(unsafe { ptr.offset(0x8) } as _),
        }
    }

    pub fn get_entity_id(&self) -> Result<i32> {
        type Func = unsafe extern "thiscall" fn(*const usize) -> i32;

        Ok(unsafe {
            transmute::<_, Func>(
                self.entity_id_this
                    .nth(ClientEntityVTableIndicies::GetEntityId as isize)?,
            )(*self.entity_id_this.handle())
        })
    }
}
