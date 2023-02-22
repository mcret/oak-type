//! Inputs are things that a person or other device (such as the host) can do to cause a behavior.
//!
//! Examples include:
//! * Analog inputs from hall effect sensors or a varistor
//! * Momentary switches from a traditional keyboard switch
//! * Messages from software on a host machine or other device

use alloc::string::String;
use crate::signal::Signal;

/// Inputs represent physical or logical inputs to the keyboard.
/// They can be anything from a momentary switch seen in typical keyboards
/// to analog inputs from hall effect sensors or a varistor, to inputs from software on a host
/// machine
pub trait Input
{
    /// Returns the signal from the hardware
    fn get_input(&self) -> Signal;
}

pub struct GenericInput
{
    input_getter: fn() -> Signal,
}

impl Input for GenericInput
{
    fn get_input(&self) -> Signal {
        (self.input_getter)()
    }
}

pub struct SimpleConfig
{
    id: String,
    pin: u16,
}

/// Marker trait indicating that the implementing struct can be used as a configuration
/// for an input of type _I_
pub trait InputConfig<I> where I: Input {}

/// Defines (typically at the hardware level) how a configured input is read.
///
/// For example:
/// * For an input configuration _IC_, what does the hardware do to _read the signal for I_?
/// * For an input configuration _AnalogConfigU16_, what does the hardware do to _read a u16_?
pub trait InputConfigurator<IC, I>
    where
        IC: InputConfig<I>,
        I: Input,
{
    fn configure(&self, in_conf: IC) -> dyn Input;
}