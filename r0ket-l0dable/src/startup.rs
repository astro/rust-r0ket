use r0;

/// Start-up
///
/// * Zeroes the `.bss` segment
/// * Calls your main function which must be defined as `#[no_mangle] pub extern "C" fn ram()`
///
/// Gets linked in first by the `l0dable.x` linker script.
#[no_mangle]
#[link_section = ".startup"]
pub extern "C" fn start() {
    extern "C" {
        static mut __sbss: u32;
        static mut __ebss: u32;
        fn ram();
    }
    unsafe {
        // Initialize RAM
        r0::zero_bss(&mut __sbss, &mut __ebss);

        ram();
    }
}
