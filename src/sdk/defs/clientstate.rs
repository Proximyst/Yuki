use super::super::prelude::*;
use getset::Getters;
use std::mem::transmute;

#[derive(Clone, Getters)]
#[get = "pub"]
#[repr(C)]
#[allow(non_snake_case)]
pub struct ClientState {
    pad_0000: [u8; 156],
    net_channel: u32,
    challenge_count: u32,
    reconnect_time: f64,
    retry_count: i32,
    pad_00A8: [u8; 88],
    signon_state_count: i32,
    pad_0104: [u8; 8],
    next_cmd_time: f64,
    server_count: i32,
    current_sequence: u32,
    pad_0118: [u8; 8 + 0x4C],
    delta_tick: i32,
    is_paused: bool,
    pad_0171: [u8; 3],
    view_entity: i32,
    player_slot: i32,
    pad_017C: [u8; 4],
    level_name: [u8; 260],
    level_name_short: [u8; 40],
    pad_02AC: [u8; 92],
    max_clients: i32,
    pad_030C: [u8; 4083],
    string_table_container: u32,
    pad_1303: [u8; 14737],
    last_server_tick_time: f32,
    is_in_simulation: bool,
    pad_4C99: [u8; 3],
    old_tick_count: u32,
    tick_remainder: f32,
    frame_time: f32,
    last_outgoing_command: i32,
    choked_commands: i32,
    last_command_ack: i32,
    command_ack: i32,
    sound_sequence: i32,
    pad_4CBC: [u8; 80],
    view_angles: super::super::math::Vector3D,
}

#[derive(Clone, Getters)]
#[get = "pub"]
#[repr(C)]
#[allow(non_snake_case)]
pub struct INetChannel {
    vtable: *const *const usize,
    pad_0000: [u8; 20],
    is_processing_messages: bool,
    should_delete: bool,
    pad_0016: [u8; 2],
    out_sequence_nr: i32,
    in_sequence_nr: i32,
    out_sequence_nr_ack: i32,
    out_reliable_state_count: i32,
    in_reliable_state_count: i32,
    choked_packets: i32,
    pad_0030: [u8; 1044],
}

#[repr(isize)]
pub enum INetChannelVTableIndicies {
    Transmit = 49,
}

impl ClientState {
    pub fn full_update(&mut self) {
        self.delta_tick = -1;
    }
}

impl INetChannel {
    pub fn transmit(&self, only_reliable: bool) -> Result<bool> {
        type Func = unsafe extern "thiscall" fn(*const usize, bool) -> bool;

        Ok(unsafe {
            transmute::<_, Func>(
                *self
                    .vtable
                    .offset(INetChannelVTableIndicies::Transmit as isize),
            )(self as *const _ as _, only_reliable)
        })
    }
}
