use super::*;

/// EFI result type
pub type Result<T> = core::result::Result<T, bits::Status>;

/// Wrap a low-level status into a result
pub(crate) fn status_to_result(status: efi::bits::Status) -> Result<()> {
    if status == efi::bits::Status::SUCCESS {
        Ok(())
    } else {
        Err(status)
    }
}
