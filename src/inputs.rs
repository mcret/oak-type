mod analog;
pub mod binary;
mod matrix;
mod multiplex;

pub trait Input
{
    type Signal;

    fn get_input(&self) -> Self::Signal;
}

pub trait InputConfig<S> {}

pub trait InputMapper<T, S>
    where
        T: InputConfig<S>,
{
    fn configure(&self, foo: T) -> fn() -> S;
}