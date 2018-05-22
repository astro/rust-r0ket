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

extern crate lpc13xx as target;
extern crate r0;
extern crate embedded_hal as hal;

pub mod startup;
pub mod table;
pub mod lcd;
pub mod input;
pub mod led;
pub mod time;
pub mod rand;
pub mod power;
