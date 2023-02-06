#![no_std]
#![no_main]

use embedded_hal::digital::v2::OutputPin;
use panic_halt as _;
use seeeduino_xiao_rp2040::entry;
use seeeduino_xiao_rp2040::hal;
use seeeduino_xiao_rp2040::hal::pac;
use seeeduino_xiao_rp2040::hal::prelude::*;
use embedded_alloc::Heap;

#[global_allocator]
static HEAP: Heap = Heap::empty();

pub mod models;
// mod utils;

#[entry]
fn main() -> ! {
    loop {

    }
}
