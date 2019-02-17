use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    Unknown,
    OutOfResources,
    InvalidParameter,
}

pub type Result<T> = core::result::Result<T, Error>;

pub(crate) fn status_to_result(status: efi::bits::Status) -> Result<()> {
    if status == efi::bits::Status::SUCCESS {
        Ok(())
    }
    else {
        match status {
            efi::bits::Status::OUT_OF_RESOURCES => Err(Error::OutOfResources),
            efi::bits::Status::INVALID_PARAMETER => Err(Error::InvalidParameter),
            _ => Err(Error::Unknown),
        }
    }
}
