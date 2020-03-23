mod command;
mod dispatcher;
mod graph;
mod help;
mod literal;
mod parser;
mod suggestions;
mod syntax;

pub use command::Command;
pub use dispatcher::Dispatcher;
pub use graph::{Node, Root};
pub use help::Help;
pub use literal::Literal;
pub use parser::{Parser, Result as ParseResult};
pub use suggestions::Suggestions;
pub use syntax::Syntax;

pub type Input<'a> = &'a str;

#[derive(Debug, PartialEq)]
pub enum Error<'a> {
    Parser(&'a str),
}

#[derive(Debug, PartialEq)]
pub enum CommandResult {
    Successful,
    Failed,
    Effected(usize),
}

impl Default for CommandResult {
    fn default() -> Self {
        CommandResult::Successful
    }
}

#[cfg(test)]
mod tests {
    use crate::commands::{Command, CommandResult, Dispatcher, Error, Node, Root};
    #[test]
    fn command() {
        let command = Command::new(|_: (i32,)| CommandResult::Successful);

        assert_eq!(command.call(&mut (), "abc"), Err(Error::Parser("abc")));
        assert_eq!(command.call(&mut (), "10"), Ok(CommandResult::Successful));
    }

    #[test]
    fn command_multiple_arguments() {
        let command = Command::new(|_: (i32, f32)| CommandResult::Successful);

        assert_eq!(
            command.call(&mut (), "10 4.2"),
            Ok(CommandResult::Successful)
        );
    }

    #[test]
    fn command_graph() {
        let foo = Command::new(|_: ()| CommandResult::Successful);
        let bar = Command::new(|_: ()| CommandResult::Effected(10));
        let graph = Root::from(vec![
            Node::literal("foo", vec![Node::command(foo)]),
            Node::literal("bar", vec![Node::command(bar)]),
        ]);

        assert_eq!(graph.call(&mut (), "foo"), Ok(CommandResult::Successful));
        assert_eq!(graph.call(&mut (), "bar"), Ok(CommandResult::Effected(10)));
    }
}
