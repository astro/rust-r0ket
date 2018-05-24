use core::slice;
use core::fmt;

use table::table;

/// Display width in pixels
pub const RESX: usize = 96;
/// Display height in pixels
pub const RESY: usize = 68;
/// `RESY` in bytes
pub const RESY_B: usize = 9;


/// Obtain the display buffer
///
/// Changes must be flushed with `display()`
pub fn buffer() -> &'static mut [u8] {
    unsafe { slice::from_raw_parts_mut(table().lcdBuffer, RESX * RESY_B) }
}

/// Set/clear pixel at `x`×`y`
pub fn set_pixel(x: i8, y: i8, f: bool) {
    (table().lcdSetPixel)(x, y, f);
}

/// Get pixel at `x`×`y`
pub fn get_pixel(x: i8, y: i8) -> bool {
    (table().lcdGetPixel)(x, y)
}

/// Print an integer `num` at position `sx`×`sy`
pub fn do_int(sx: isize, sy: isize, num: isize) {
    (table().DoInt)(sx, sy, num);
}

/// Print a string
///
/// A string `s` of up to 15 bytes will be copied into
/// nul-terminated buffer.
pub fn print(s: &str) {
    let mut buf = [0u8; 16];
    let len = s.len().min(buf.len());
    buf[0..len].copy_from_slice(&s.as_bytes().as_ref()[0..len]);
    (table().lcdPrint)(buf.as_ptr());
}

/// Print a line
///
/// A string `s` of up to 15 bytes will be copied into
/// nul-terminated buffer.
pub fn println(s: &str) {
    let mut buf = [0u8; 16];
    let len = s.len().min(buf.len());
    buf[0..len].copy_from_slice(&s.as_bytes().as_ref()[0..len]);
    (table().lcdPrintln)(buf.as_ptr());
}

/// Writes onto the display
pub struct Stdout;
/// Do not use as of now. Linking in such functionality makes the l0dable exceed 2560 bytes.
pub const STDOUT: Stdout = Stdout;

impl fmt::Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let mut bytes = &s.as_bytes().as_ref()[..];
        while bytes.len() > 0 {
            let mut buf = [0u8; 16];
            let len = s.len().min(buf.len() - 1);
            buf[0..len].copy_from_slice(&bytes[..len]);
            (table().lcdPrint)(buf.as_ptr());
            
            bytes = &bytes[len..];
        }
        Ok(())
    }
}

/// Clear the display buffer
pub fn clear() {
    (table().lcdClear)();
}

/// Flush the display buffer to hardware
pub fn display() {
    (table().lcdDisplay)();
}
