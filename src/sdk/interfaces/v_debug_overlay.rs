use std::{
    ffi::c_void,
    mem::{self, transmute},
};

use getset::Getters;

use super::super::math::Vector3D;
use super::super::prelude::*;

#[derive(Debug, Copy, Clone, Getters)]
#[get = "pub"]
pub struct VDebugOverlayInterface {
    inner: Interface,
}

#[repr(isize)]
pub enum VDebugOverlayVTableIndicies {
    WorldToScreen = 13,
}

impl VDebugOverlayInterface {
    pub const fn new(inner: Interface) -> Self {
        VDebugOverlayInterface { inner }
    }

    pub fn world_to_screen(&self, input: &Vector3D) -> Result<Option<Vector3D>> {
        type Func =
            unsafe extern "thiscall" fn(*const usize, *const Vector3D, *mut Vector3D) -> i32;

        let mut vector: Vector3D = unsafe { mem::zeroed() };
        let return_code = unsafe {
            transmute::<_, Func>(
                self.inner
                    .nth(VDebugOverlayVTableIndicies::WorldToScreen as isize)?,
            )(
                *self.inner.handle(),
                input as *const _,
                &mut vector as *mut _,
            )
        };

        if return_code == 1 {
            return Ok(None);
        }

        Ok(Some(vector))
    }
}
