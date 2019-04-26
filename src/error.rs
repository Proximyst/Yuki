use failure::{Error as FError, Fail};
use std::result::Result as StdResult;

pub type Result<T> = StdResult<T, FError>;

#[derive(Debug, Fail)]
pub enum ProcessErrorKind {
    #[fail(display = "Couldn't read memory at {:X}", _0)]
    MemoryRead(u32),

    #[fail(display = "CreateToolhelp32Snapshot returned INVALID_HANDLE_VALUE")]
    InvalidHandleValue,

    #[fail(display = "Unknown module: {}", _0)]
    UnknownModule(String),

    #[fail(display = "Couldn't write to {:X}", _0)]
    InvalidBytesWritten(u32),

    #[fail(display = "Unknown export: {}", _0)]
    UnknownExport(String),

    #[fail(display = "Unknown interface: {}", _0)]
    UnknownInterface(String),
}

#[derive(Debug, Fail)]
pub enum InterfaceErrorKind {
    #[fail(display = "Invalid vfunction index: {}", _0)]
    InvalidVFuncIndex(isize),
}

#[derive(Debug, Fail)]
pub enum StdErrorKind {
    #[fail(display = "Option value not present")]
    None,
}

pub trait OptionExt<T> {
    fn failure(self) -> Result<T>;
}

impl<T> OptionExt<T> for Option<T> {
    fn failure(self) -> Result<T> {
        self.ok_or(StdErrorKind::None.into())
    }
}
