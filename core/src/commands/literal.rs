use crate::commands::{Input, ParseResult, Parser};

#[derive(Debug, PartialEq)]
pub struct Literal<'a>(pub &'a str);

impl<'a, T> From<T> for Literal<'a>
where
    T: Into<&'a str>,
{
    fn from(literal: T) -> Self {
        Literal(literal.into())
    }
}

impl<'a, C> Parser<'a, C> for Literal<'a> {
    fn parse(_: &mut C, input: Input<'a>) -> ParseResult<'a, Self> {
        let (head, tail) = {
            let mut split = input.splitn(2, " ");
            (split.next().unwrap_or(""), split.next().unwrap_or(""))
        };
        Ok((Literal(head), tail))
    }
}
