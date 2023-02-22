use crate::behaviors::Behavior;
use crate::signal::Signal;

pub trait Interpreter
{
    /// Interpret what should be done with the signal from the Input
    fn interpret(&self, input: Signal) -> ();
}

/// Defines what should be done with a signal
pub trait MappingConfigurator<B>
    where B: Behavior
{
    fn configure(&self, signal: Signal, behavior: B) -> fn(Signal) -> ();
}