use super::*;

struct StdOut;

impl core::fmt::Write for StdOut {
    fn write_str(&mut self, string: &str) -> core::fmt::Result {
        let mut con_out = efi::SystemTable::get().con_out();
        ucs2::encode_with(string, |ch| {
            let mut buffer: [u16; 2] = [0, 0];
            buffer[0] = ch;
            unsafe {
                con_out.output_string(&buffer[0] as *const u16 as _).map_err(|_| ucs2::Error::InvalidData)
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
