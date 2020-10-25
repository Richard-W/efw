use super::*;
use core::convert::TryFrom;

/// Type of a memory region
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryType {
    Efi(MemoryTypeEfi),
    Oem(MemoryTypeOem),
    Os(MemoryTypeOs),
}

impl TryFrom<u32> for MemoryType {
    type Error = bits::Status;

    fn try_from(val: u32) -> Result<Self> {
        MemoryTypeEfi::try_from(val)
            .map(MemoryType::Efi)
            .or_else(|_| MemoryTypeOem::try_from(val).map(MemoryType::Oem))
            .or_else(|_| MemoryTypeOs::try_from(val).map(MemoryType::Os))
    }
}

impl Into<u32> for MemoryType {
    fn into(self) -> u32 {
        match self {
            Self::Efi(x) => x.into(),
            Self::Oem(x) => x.into(),
            Self::Os(x) => x.into(),
        }
    }
}

/// Base EFI memory type
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryTypeEfi {
    ReservedMemoryType = 0,
    LoaderCode = 1,
    LoaderData = 2,
    BootServicesCode = 3,
    BootServicesData = 4,
    RuntimeServicesCode = 5,
    RuntimeServicesData = 6,
    ConventionalMemory = 7,
    UnusableMemory = 8,
    AcpiReclaimMemory = 9,
    AcpiMemoryNvs = 10,
    MemoryMappedIO = 11,
    MemoryMappedIOPortSpace = 12,
    PalCode = 13,
    PersistentMemory = 14,
}

const MAX_MEMORY_TYPE: u32 = 15;

impl TryFrom<u32> for MemoryTypeEfi {
    type Error = bits::Status;

    fn try_from(val: u32) -> Result<Self> {
        if val < MAX_MEMORY_TYPE {
            Ok(unsafe { core::mem::transmute(val) })
        } else {
            Err(bits::Status::INVALID_PARAMETER)
        }
    }
}

impl Into<u32> for MemoryTypeEfi {
    fn into(self) -> u32 {
        unsafe { core::mem::transmute(self) }
    }
}

/// Reserved memory type range for OEMs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemoryTypeOem(u32);

impl TryFrom<u32> for MemoryTypeOem {
    type Error = bits::Status;

    fn try_from(val: u32) -> Result<Self> {
        if 0x7000_0000 <= val && val <= 0x7fff_ffff {
            Ok(MemoryTypeOem(val))
        } else {
            Err(bits::Status::INVALID_PARAMETER)
        }
    }
}

impl Into<u32> for MemoryTypeOem {
    fn into(self) -> u32 {
        self.0
    }
}

/// Reserved memory type range for OSs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemoryTypeOs(u32);

impl TryFrom<u32> for MemoryTypeOs {
    type Error = bits::Status;

    fn try_from(val: u32) -> Result<Self> {
        if 0x8000_0000 <= val {
            Ok(MemoryTypeOs(val))
        } else {
            Err(bits::Status::INVALID_PARAMETER)
        }
    }
}

impl Into<u32> for MemoryTypeOs {
    fn into(self) -> u32 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn efi_memory_type_values() {
        macro_rules! assert_value {
            ($name:tt) => {
                let high: u32 = MemoryTypeEfi::$name.into();
                let low: u32 = unsafe { core::mem::transmute(bits::MemoryType::$name) };
                assert_eq!(high, low);
            };
        }
        assert_value!(ReservedMemoryType);
        assert_value!(LoaderCode);
        assert_value!(LoaderData);
        assert_value!(BootServicesCode);
        assert_value!(BootServicesData);
        assert_value!(RuntimeServicesCode);
        assert_value!(RuntimeServicesData);
        assert_value!(ConventionalMemory);
        assert_value!(UnusableMemory);
        assert_value!(AcpiReclaimMemory);
        assert_value!(AcpiMemoryNvs);
        assert_value!(MemoryMappedIO);
        assert_value!(MemoryMappedIOPortSpace);
        assert_value!(PalCode);
        assert_value!(PersistentMemory);
    }

    #[test]
    fn efi_memory_type_from_u32() {
        assert_eq!(
            MemoryType::try_from(14),
            Ok(MemoryType::Efi(MemoryTypeEfi::PersistentMemory))
        );
        assert_eq!(
            MemoryType::try_from(15),
            Err(bits::Status::INVALID_PARAMETER)
        );
        assert_eq!(
            MemoryType::try_from(0x6fff_ffff),
            Err(bits::Status::INVALID_PARAMETER)
        );
        assert_eq!(
            MemoryType::try_from(0x7000_0000),
            Ok(MemoryType::Oem(
                MemoryTypeOem::try_from(0x7000_0000).unwrap()
            ))
        );
        assert_eq!(
            MemoryType::try_from(0x7fff_ffff),
            Ok(MemoryType::Oem(
                MemoryTypeOem::try_from(0x7fff_ffff).unwrap()
            ))
        );
        assert_eq!(
            MemoryType::try_from(0x8000_0000),
            Ok(MemoryType::Os(MemoryTypeOs::try_from(0x8000_0000).unwrap()))
        );
        assert_eq!(
            MemoryType::try_from(0xffff_ffff),
            Ok(MemoryType::Os(MemoryTypeOs::try_from(0xffff_ffff).unwrap()))
        );
    }
}
