//! Behaviors are things that the device can do, either internally in the firmware, or externally.
//!
//! Examples:
//! * Changing a layer
//! * Sending a character to a host

pub trait Behavior
{
    fn execute(&self) -> ();
}

/// Marker trait indicating that the implementing struct can be used as an Behavior Configuration
///  (OC) which will output a behavior of type B
pub trait BehaviorConfig<B> {}

/// Defines (typically at the hardware level) how an output behavior is implemented
///
/// For example:
/// * For an output configuration _OC_, what does the hardware do to _do the behavior B_?
/// * For an output configuration _HID_, what does the hardware do to _send an "a" to the host_?
pub trait BehaviorConfigurator<BC, B>
    where
        BC: BehaviorConfig<B>,
{
    fn configure(&self, behavior_conf: BC) -> fn() -> ();
}

/// Used for outputs from the device such as sending a character to the host, making a sound,
/// sending a message to a display, or changing the state of an led
pub mod outputs
{
    pub mod hid;
}

/// Used for actions that affect how the device is running, such as changing a layer or resetting
pub mod internal
{
    pub mod layer_switch;
}