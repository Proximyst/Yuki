use super::super::math::Vector3D;
use bitflags::bitflags;

bitflags! {
    #[repr(transparent)]
    pub struct CmdButtons: i32 {
        const IN_ATTACK = 1 << 0;
        const IN_JUMP = 1 << 1;
        const IN_DUCK = 1 << 2;
        const IN_FORWARD = 1 << 3;
        const IN_BACK = 1 << 4;
        const IN_USE = 1 << 5;
        const IN_CANCEL = 1 << 6;
        const IN_LEFT = 1 << 7;
        const IN_RIGHT = 1 << 8;
        const IN_MOVE_LEFT = 1 << 9;
        const IN_MOVE_RIGHT = 1 << 10;
        const IN_ATTACK_2 = 1 << 11;
        const IN_RUN = 1 << 12;
        const IN_RELOAD = 1 << 13;
        const IN_ALT_1 = 1 << 14;
        const IN_ALT_2 = 1 << 15;
        const IN_SCOREBOARD = 1 << 16;
        const IN_SPEED = 1 << 17;
        const IN_WALK = 1 << 18;
        const IN_ZOOM = 1 << 19;
        const IN_WEAPON_1 = 1 << 20;
        const IN_WEAPON_2 = 1 << 21;
        const IN_BULL_RUSH = 1 << 22;
        const IN_GRENADE_1 = 1 << 23;
        const IN_GRENADE_2 = 1 << 24;
        const IN_ATTACK_3 = 1 << 25;
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct CUserCmd {
    pub destructor: *const *const fn(),
    pub command_number: i32,
    pub tick_count: i32,
    pub view_angles: Vector3D,
    pub aim_direction: Vector3D,
    pub forward_move: f32,
    pub side_move: f32,
    pub up_move: f32,
    pub buttons: CmdButtons,
    pub impulse: u8,
    pub weapon_select: i32,
    pub weapon_subtype: i32,
    pub random_seed: i32,
    pub mouse_dx: i16,
    pub mouse_dy: i16,
    pub has_been_predicted: bool,
    pub pad: [u8; 0x18],
}

#[derive(Clone)]
#[repr(transparent)]
pub struct CVerifiedUserCmd {
    pub user_cmd: CUserCmd,
}

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(i32)]
pub enum ButtonCode {
    KeyNone = 0,
    Key0,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    KeyA,
    KeyB,
    KeyC,
    KeyD,
    KeyE,
    KeyF,
    KeyG,
    KeyH,
    KeyI,
    KeyJ,
    KeyK,
    KeyL,
    KeyM,
    KeyN,
    KeyO,
    KeyP,
    KeyQ,
    KeyR,
    KeyS,
    KeyT,
    KeyU,
    KeyV,
    KeyW,
    KeyX,
    KeyY,
    KeyZ,
    KeyPad0,
    KeyPad1,
    KeyPad2,
    KeyPad3,
    KeyPad4,
    KeyPad5,
    KeyPad6,
    KeyPad7,
    KeyPad8,
    KeyPad9,
    KeyPadDivide,
    KeyPadMultiply,
    KeyPadMinus,
    KeyPadPlus,
    KeyPadEnter,
    KeyPadDecimal,
    KeyLBracket,
    KeyRBracket,
    KeySemicolon,
    KeyApostrophe,
    KeyBackQuote,
    KeyComma,
    KeyPeriod,
    KeySlash,
    KeyBackslash,
    KeyMinus,
    KeyEqual,
    KeyEnter,
    KeySpace,
    KeyBackspace,
    KeyTab,
    KeyCapsLock,
    KeyNumLock,
    KeyEscape,
    KeyScrollLock,
    KeyInsert,
    KeyDelete,
    KeyHome,
    KeyEnd,
    KeyPageUp,
    KeyPageDown,
    KeyBreak,
    KeyLShift,
    KeyRShift,
    KeyLAlt,
    KeyRAlt,
    KeyLControl,
    KeyRControl,
    KeyLWin,
    KeyRWin,
    KeyApp,
    KeyUp,
    KeyLeft,
    KeyDown,
    KeyRight,
    KeyF1,
    KeyF2,
    KeyF3,
    KeyF4,
    KeyF5,
    KeyF6,
    KeyF7,
    KeyF8,
    KeyF9,
    KeyF10,
    KeyF11,
    KeyF12,
    KeyCapsLockToggle,
    KeyNumLockToggle,
    KeyScrollLockToggle,

    MouseLeft,
    MouseRight,
    MouseMiddle,
    Mouse4,
    Mouse5,
    MouseWheelUp,
    MouseWheelDown,
}

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(i32)]
pub enum MouseCodeState {
    ButtonReleased,
    ButtonPressed,
    ButtonDoubleClicked,
}

impl ButtonCode {
    pub fn to_i32(&self) -> i32 {
        *self as i32
    }

    pub fn from_i32(i: i32) -> Option<ButtonCode> {
        if i < Self::first_key().to_i32() || i > (Self::key_count() + Self::mouse_count()) {
            None
        } else {
            Some(unsafe { std::mem::transmute(i) })
        }
    }

    pub fn first_key() -> ButtonCode {
        ButtonCode::KeyNone
    }

    pub fn last_key() -> ButtonCode {
        ButtonCode::KeyScrollLockToggle
    }

    pub fn key_count() -> i32 {
        Self::last_key().to_i32() - Self::first_key().to_i32() + 1
    }

    pub fn first_mouse() -> ButtonCode {
        ButtonCode::MouseLeft
    }

    pub fn last_mouse() -> ButtonCode {
        ButtonCode::MouseWheelDown
    }

    pub fn mouse_count() -> i32 {
        Self::last_mouse().to_i32() - Self::first_mouse().to_i32() + 1
    }
}
