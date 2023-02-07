#![no_std]
#![no_main]

extern crate alloc;

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

#[entry]
fn main() -> ! {
    loop {
        load_config();
        loop {
            update_state();
            perform_behaviors();
            if config_updates()
            {
                break;
            } else {}
        }
    }
}

fn config_updates() -> bool {
    todo!()
}

fn perform_behaviors() {
    todo!()
}

fn update_state() {
    todo!()
}

fn load_config() {
    todo!()
}
