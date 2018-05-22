use table::table;
use hal::blocking::delay::DelayMs;

pub fn get_seconds() -> u32 {
    (table().getSeconds)()
}

pub fn get_ticks() -> u32 {
    (table().systickGetTicks)()
}

pub fn delay_ms(ms: u32) {
    (table().delayms)(ms)
}

pub struct Delay;
impl<T: Into<u32>> DelayMs<T> for Delay {
    fn delay_ms(&mut self, ms: T) {
        self::delay_ms(ms.into())
    }
}
