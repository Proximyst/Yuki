use std::ffi::c_void;

#[repr(C)]
pub struct CGlobalVars {
    pub realtime: f32,
    pub framecount: i32,
    pub absolute_frame_time: f32,
    pub absolute_frame_start_time_std_dev: f32,
    pub curtime: f32,
    pub frametime: f32,
    pub max_clients: i32,
    pub tickcount: i32,
    pub interval_per_tick: f32,
    pub interpolation_amount: f32,
    pub sim_ticks_this_frame: i32,
    pub network_protocol: i32,
    pub save_data: *const c_void,
    pub client: bool,
    pub remote_client: bool,
}
