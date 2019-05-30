use super::super::prelude::*;
use super::super::Vector3D;
use std::mem::transmute;

pub struct Collideable {
    base: VTable,
}

impl Collideable {
    pub fn mins(&self) -> Result<&mut Vector3D> {
        type Func = unsafe extern "thiscall" fn(*const usize) -> *mut Vector3D;

        Ok(unsafe { &mut *transmute::<_, Func>(self.base.nth(1)?)(*self.base.handle()) })
    }

    pub fn maxs(&self) -> Result<&mut Vector3D> {
        type Func = unsafe extern "thiscall" fn(*const usize) -> *mut Vector3D;

        Ok(unsafe { &mut *transmute::<_, Func>(self.base.nth(2)?)(*self.base.handle()) })
    }
}
