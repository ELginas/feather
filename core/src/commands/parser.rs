use crate::commands::{Error, Input};
use std::str::FromStr;

pub type Result<'a, T> = std::result::Result<(T, Input<'a>), Error<'a>>;

pub trait Parser<'a, C>: Sized {
    fn parse(context: &mut C, input: Input<'a>) -> Result<'a, Self>;
}

impl<'a, C> Parser<'a, C> for () {
    fn parse(_: &mut C, input: Input<'a>) -> Result<'a, Self> {
        Ok(((), input))
    }
}

impl<'a, C, T> Parser<'a, C> for (T,)
where
    T: FromStr,
{
    fn parse(_: &mut C, input: Input<'a>) -> Result<'a, Self> {
        let (head, tail) = {
            let mut split = input.splitn(2, " ");
            (split.next().unwrap_or(""), split.next().unwrap_or(""))
        };
        match head.parse() {
            Ok(value) => Ok(((value,), tail)),
            Err(_) => Err(Error::Parser(head)),
        }
    }
}

macro_rules! impl_parser {
    ($($typ:ident),*) => {
        impl <'a, C, $($typ, )*> Parser<'a, C> for ($($typ, )*) where $(($typ,): Parser<'a, C>,)* {
            #[allow(non_snake_case)]
            fn parse(context: &mut C, input: Input<'a>) -> Result<'a, Self> {
                let tail = input;
                $(
                    let (($typ,), tail) = <($typ,)>::parse(context, tail)?;
                )*

                Ok((($($typ,)*), tail))
            }
        }
    }
}

macro_rules! recursive_macro_call_on_tuple {
    ($m: ident, $ty: ident) => {

    };
    ($m: ident, $ty: ident, $($tt: ident),*) => {
        $m!{$ty, $($tt),*}
        recursive_macro_call_on_tuple!{$m, $($tt),*}
    };
}

recursive_macro_call_on_tuple!(
    impl_parser,
    T1,
    T2,
    T3,
    T4,
    T5,
    T6,
    T7,
    T8,
    T9,
    T10,
    T11,
    T12,
    T13,
    T14,
    T15,
    T16,
    T17,
    T18,
    T19,
    T20
);
