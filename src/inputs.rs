//! Inputs are things that a person or other device (such as the host) can do to cause a behavior.
//!
//! Examples include:
//! * Analog inputs from hall effect sensors or a varistor
//! * Momentary switches from a traditional keyboard switch
//! * Messages from software on a host machine or other device

pub mod analog;
pub mod binary;
mod matrix;
mod multiplex;

/// Structs that implement the Input trait represent physical or logical inputs
/// to the keyboard. They can be anything from a momentary switch seen in typical keyboards
/// to analog inputs from hall effect sensors or a varistor, to inputs from software on a host
/// machine
pub trait Input
{
    type Signal;

    fn get_input(&self) -> Self::Signal;
}

/// Marker trait indicating that the implementing struct can be used as a configuration
/// which read a signal of type S
pub trait InputConfig<S> {}

/// Defines (typically at the hardware level) how a configured input is read.
///
/// For example:
/// * For an input configuration _IC_, what does the hardware do to _read the signal S_?
/// * For an input  configuration _analog GPIO2_, what does the hardware do to _read GPIO2_?
pub trait InputConfigurator<IC, S>
    where
        IC: InputConfig<S>,
{
    fn configure(&self, in_conf: IC) -> fn() -> S;
}