use std::mem::transmute;

use getset::Getters;

use super::super::defs::c_usercmd::CUserCmd;
use super::super::prelude::*;

#[derive(Debug, Copy, Clone, Getters)]
#[get = "pub"]
pub struct InputInterface {
    inner: Interface,
}

#[repr(isize)]
pub enum InputVTableIndicies {
    GetUserCmd = 8,
}

impl InputInterface {
    pub const fn new(inner: Interface) -> Self {
        InputInterface { inner }
    }

    pub fn get_user_cmd(&self, slot: i32, seq_num: i32) -> Result<*const CUserCmd> {
        type Func = unsafe extern "thiscall" fn(*const usize, i32, i32) -> *const CUserCmd;

        Ok(unsafe {
            transmute::<_, Func>(self.inner.nth(InputVTableIndicies::GetUserCmd as isize)?)(
                *self.inner.handle(),
                slot,
                seq_num,
            )
        })
    }
}
