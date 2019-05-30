use super::super::prelude::*;
use std::mem::transmute;
use getset::Getters;

#[derive(Debug, Copy, Clone, Getters)]
#[get = "pub"]
pub struct PanelInterface {
    inner: Interface,
}

#[repr(isize)]
pub enum PanelVTableIndicies {
    GetName = 36,
}

impl PanelInterface {
    pub const fn new(inner: Interface) -> Self {
        PanelInterface { inner }
    }

    pub fn get_name(&self, vgui_panel: u32) -> Result<*const libc::c_char> {
        type Func = unsafe extern "thiscall" fn(*const usize, u32) -> *const libc::c_char;

        Ok(unsafe {
            transmute::<_, Func>(self.inner.nth(PanelVTableIndicies::GetName as isize)?)(
                *self.inner.handle(),
                vgui_panel,
            )
        })
    }
}
