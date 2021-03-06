//! Write **l0dable** apps for the [CCCamp 2011](https://events.ccc.de/camp/2011/) [r0ket badge](https://github.com/r0ket/r0ket).
//!
//! ## Prerequisites
//!
//! ### File `.cargo/config`
//!
//! ```toml
//! [target.thumbv7m-none-eabi]
//! runner = "arm-none-eabi-gdb"
//! rustflags = [
//!   "-C", "link-arg=-Tl0dable.x",
//!   "-C", "linker=lld",
//!   "-Z", "linker-flavor=ld.lld",
//! ]
//!
//! [build]
//! target = "thumbv7m-none-eabi"
//! ```
//!
//! ### Enable Link-Time Optimization in `Cargo.toml`
//!
//! Optional, but strongly recommended for code size:
//!
//! ```toml
//! [profile.release]
//! lto = true
//! ```
//! ### Build script
//!
//! ```shell
//! cargo build --release
//! arm-none-eabi-objcopy -O binary --strip-unneeded target/thumbv7m-none-eabi/release/demo demo.c0d
//! ```
#![no_std]
#![deny(missing_docs)]

extern crate lpc13xx as target;
extern crate r0;
extern crate embedded_hal as hal;

/// l0dable startup
pub mod startup;
/// Calling firmware functions
pub mod table;
/// Display
pub mod lcd;
/// Joystick
pub mod input;
/// Blinking LEDs
pub mod led;
/// Time functions
pub mod time;
/// `getRandom`
pub mod rand;
/// Power supply information
pub mod power;
/// nRF24l01 radio communications
pub mod nrf;
