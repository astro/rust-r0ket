#![no_std]
#![no_main]

extern crate cortex_m;
extern crate panic_abort;
extern crate lpc13xx as target;
#[macro_use(entry)]
extern crate r0ket_l0dable as r0ket;
extern crate embedded_hal as hal;

// use core::fmt::Write;
use r0ket::{lcd, input, led, time, nrf, rand};


entry!(ram);

fn ram() {
    nrf::set_config(&nrf::Config {
        channel: 8,
        txmac: *b"astro",
        mac0: *b"astro",
        mac1: *b"astro",
        mac2345: *b"oooo",
        nrmacs: 1,
        maclen: [31; 5],
    });
    let rx = nrf::rx(false);

    let mut running = true;
    let mut pkts = 0usize;
    lcd::clear();
    lcd::println("Image Receiver");
    lcd::println(" ");
    lcd::println("Waiting for");
    lcd::println("packets...");
    lcd::display();

    while running {
        led::set_led(2, (time::get_ticks() & 0x3f) <= 0x3);

        led::set_led(1, true);
        loop {
            match rx.poll() {
                Err(nrf::RxError::WouldBlock) =>
                    break,
                Err(_) => {
                    running = false;
                    break;
                },
                Ok(payload) => {
                    led::set_led(3, true);
                    if pkts == 0 {
                        lcd::clear();
                    }
                    pkts += 1;
                    handle_packet(payload.as_ref());
                    led::set_led(3, false);
                },
            }
        }
        if pkts == 0 {
            // Draw no signal screen
            for b in lcd::buffer().iter_mut() {
                *b = rand::get_random() as u8;
            }
            if time::get_seconds() & 1 == 0 {
                let s = "no signal";
                lcd::do_string(
                    lcd::RESX / 2 - 7 * s.len() as isize / 2,
                    lcd::RESY / 2 - 7 / 2,
                    s
                );
            }
        }
        lcd::display();
        led::set_led(1, false);

        if input::get_input_raw().enter() {
            running = false;
        }
    }
}

fn handle_packet(buf: &[u8]) {
    if buf.len() >= 5 {
        let mut x = buf[0];
        let y = buf[1];
        for b in &buf[2..] {
            for j in 0..8 {
                let set = *b & (0x80 >> j) != 0;
                lcd::set_pixel(x as i8, y as i8, set);
                x += 1;
            }            
        }
    }
}
