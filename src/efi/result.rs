use super::*;

pub type Result<T> = core::result::Result<T, bits::Status>;

pub(crate) fn status_to_result(status: efi::bits::Status) -> Result<()> {
    if status == efi::bits::Status::SUCCESS {
        Ok(())
    }
    else {
        Err(status)
    }
}
