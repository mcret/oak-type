/// Defines what behavior _B_ should be associated to signal _S_
pub trait MappingConfigurator<S, B>
{
    //todo should this return Option<B>
    fn configure(&self, signal: S, behavior: B) -> fn(S) -> B;
}