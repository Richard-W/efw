use super::*;

struct StdOut;

impl core::fmt::Write for StdOut {
    fn write_str(&mut self, string: &str) -> core::fmt::Result {
        let con_out = unsafe { &mut *(*SYSTEM_TABLE.native()).con_out };
        ucs2::encode_with(string, |ch| {
            let mut buffer: [u16; 2] = [0, 0];
            buffer[0] = ch;
            if (con_out.output_string)(con_out as _, &buffer[0] as *const u16 as _) != efi::bits::Status::SUCCESS {
                Err(ucs2::Error::InvalidData)
            }
            else {
                Ok(())
            }
        }).unwrap();
        Ok(())
    }
}

#[doc(hidden)]
pub fn _print_stdout(args: core::fmt::Arguments) {
    use core::fmt::Write;
    StdOut.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::_print_stdout(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => (print!("\r\n"));
    ($($arg:tt)*) => (print!("{}\r\n", format_args!($($arg)*)));
}
