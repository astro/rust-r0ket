use target::{Peripherals, GPIO0, GPIO1};

macro_rules! def_gpio_setter {
    ($name: ident, $GPIO: ident) => (
        fn $name(gpio: &$GPIO, pos: usize, set: bool) {
            let mask = 1 << pos;
            gpio.data.modify(|r, w| {
                let bits = if !set {
                    r.bits() & !mask
                } else {
                    r.bits() | mask
                };
                unsafe { w.bits(bits) }
            });
        }
    )
}
def_gpio_setter!(set_gpio0, GPIO0);
def_gpio_setter!(set_gpio1, GPIO1);

/// Set/clear one of the four LEDs.
pub fn set_led(n: usize, set: bool) {
    let p = unsafe { Peripherals::steal() };
    match n & 3 {
        0 =>
            set_gpio0(&p.GPIO0, 11, set),
        1 =>
            set_gpio1(&p.GPIO1, 7, set),
        2 =>
            set_gpio1(&p.GPIO1, 6, set),
        3 =>
            set_gpio1(&p.GPIO1, 11, set),
        _ =>
            unreachable!(),
    }
}
