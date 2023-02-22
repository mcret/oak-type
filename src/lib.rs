#![no_std]
#![no_main]

//! # Open Analog Keyboard
//! ## OAK-Type
//!
//! OAK-Type's purpose is to be an analog-first firmware for keyboards and other computer input devices
extern crate alloc;

use crate::operation::Layer;

pub mod inputs;
pub mod signal;
pub mod mappers;
pub mod behaviors;

mod operation;

pub fn run(layer: Layer) -> !
{
    loop {
        layer.execute_loop();
    }
}