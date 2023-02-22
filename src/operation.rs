//! The Operation module is responsible for maintaining the state of the device, checking the inputs,
//! and triggering the associated behaviors.
//!
//! The operating loop is
//! 1. Iterate through the inputs
//! 1. Send each input's signal to the associated behavior's controller

use alloc::vec::Vec;
use crate::inputs::Input;
use crate::mappers::Interpreter;

struct Layer<'a>
{
    inputs: Vec<&'a dyn Input>,
    //todo make this return an Option
    get_interpreter: fn(&dyn Input) -> &dyn Interpreter,
}

impl Layer<'_>
{
    fn execute_loop(&self)
    {
        for input in &self.inputs
        {
            let interpreter = (self.get_interpreter)(*input);
            interpreter.interpret(input.get_input());
        }
    }
}