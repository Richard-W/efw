use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    Unknown,
    OutOfResources,
    InvalidParameter,
}

pub type Result<T> = core::result::Result<T, Error>;

pub(crate) fn status_to_result(status: refi::Status) -> Result<()> {
    if status == refi::Status::SUCCESS {
        Ok(())
    }
    else {
        match status {
            refi::Status::OUT_OF_RESOURCES => Err(Error::OutOfResources),
            refi::Status::INVALID_PARAMETER => Err(Error::InvalidParameter),
            _ => Err(Error::Unknown),
        }
    }
}
