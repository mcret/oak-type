#![no_std]
#![no_main]#![feature(type_alias_impl_trait)]

extern crate alloc;

use embedded_hal::digital::v2::OutputPin;
use embedded_alloc::Heap;
use {defmt_rtt as _, panic_probe as _};

#[global_allocator]
static HEAP: Heap = Heap::empty();

mod usb;
pub mod inputs;
pub mod outputs;

// #[entry]
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
