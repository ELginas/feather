use crate::commands::{CommandResult, Error, Input};

pub trait Dispatcher<'a, C> {
    fn call(&self, context: &mut C, input: Input<'a>) -> Result<CommandResult, Error<'a>>;
}
