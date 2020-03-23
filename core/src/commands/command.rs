use crate::commands::{CommandResult, Dispatcher, Error, Input, Parser};
use std::marker::PhantomData;

pub struct Command<'a, C, Args, F>
where
    Args: Parser<'a, C>,
    F: Fn(Args) -> CommandResult + Sized,
{
    context: PhantomData<&'a C>,
    parser: PhantomData<&'a Args>,
    command: F,
}

impl<'a, C, Args, F> Command<'a, C, Args, F>
where
    Args: Parser<'a, C>,
    F: Fn(Args) -> CommandResult + Sized,
{
    pub fn new(command: F) -> Self {
        Command {
            context: Default::default(),
            parser: Default::default(),
            command,
        }
    }
}

impl<'a, C, Args, F> Dispatcher<'a, C> for Command<'a, C, Args, F>
where
    Args: Parser<'a, C>,
    F: Fn(Args) -> CommandResult + Sized,
{
    fn call(&self, context: &mut C, input: Input<'a>) -> Result<CommandResult, Error<'a>> {
        let (args, _) = Args::parse(context, input)?;
        let command = &self.command;
        Ok(command(args))
    }
}
