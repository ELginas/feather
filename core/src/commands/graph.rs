use crate::commands::{
    CommandResult, Dispatcher, Error, Help, Input, Literal, Parser, Suggestions, Syntax,
};
use std::marker::PhantomData;

pub enum Node<'a, C> {
    Literal {
        literal: &'a str,
        children: Vec<Node<'a, C>>,
    },
    Argument {
        name: &'a str,
        children: Vec<Node<'a, C>>,
    },
    Command {
        dispatcher: Box<dyn Dispatcher<'a, C>>,
    },
}

pub struct Root<'a, C> {
    children: Vec<Node<'a, C>>,
}

impl<'a, C, N> From<N> for Root<'a, C>
where
    N: IntoIterator,
    N::Item: Into<Node<'a, C>>,
{
    fn from(nodes: N) -> Self {
        Root {
            children: nodes.into_iter().map(|e| e.into()).collect(),
        }
    }
}

impl<'a, C> Dispatcher<'a, C> for Root<'a, C> {
    fn call(&self, context: &mut C, input: Input<'a>) -> Result<CommandResult, Error<'a>> {
        self.children
            .iter()
            .find_map(|child| child.call(context, input).ok())
            .map(Result::Ok)
            .unwrap_or(Err(Error::Parser("No such command")))
    }
}

impl<'a, C> Root<'a, C> {
    pub fn registre<F, Args>(command: F)
    where
        F: Fn(Args) -> CommandResult,
        Args: Parser<'a, C>,
    {
    }
}

impl<'a, C> Node<'a, C> {
    pub fn command<D>(dispatcher: D) -> Self
    where
        D: Dispatcher<'a, C> + Sized + 'static,
    {
        Node::Command {
            dispatcher: Box::new(dispatcher),
        }
    }

    pub fn literal<L, N>(literal: L, children: N) -> Self
    where
        L: Into<&'a str>,
        N: IntoIterator,
        N::Item: Into<Node<'a, C>>,
    {
        Node::Literal {
            literal: literal.into(),
            children: children.into_iter().map(|c| c.into()).collect(),
        }
    }
}

impl<'a, C> Dispatcher<'a, C> for Node<'a, C> {
    fn call(&self, context: &mut C, input: Input<'a>) -> Result<CommandResult, Error<'a>> {
        match self {
            Node::Literal { literal, children } => {
                let (l, tail) = Literal::parse(context, input)?;
                if *literal == l.0 {
                    children
                        .iter()
                        .find_map(|child| child.call(context, tail).ok())
                        .map(Result::Ok)
                        .unwrap_or(Err(Error::Parser("No such command")))
                } else {
                    Err(Error::Parser("Literal"))
                }
            }
            Node::Command { dispatcher } => dispatcher.call(context, input),
            Node::Argument { .. } => unimplemented!(),
        }
    }
}

impl<'a, C> Suggestions<'a, C> for Node<'a, C> {
    fn suggestions(_context: &C, _input: Input) -> Vec<&'a str> {
        vec![]
    }
}

impl<'a, C> Help<C> for Node<'a, C> {
    fn help(_context: &C) -> &str {
        ""
    }
}

impl<'a, C> Syntax<C> for Node<'a, C> {
    fn syntax(_context: &C) -> &str {
        ""
    }
}
