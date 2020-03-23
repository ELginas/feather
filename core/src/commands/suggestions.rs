use crate::commands::Input;

pub trait Suggestions<'a, C> {
    // TODO return future
    fn suggestions(context: &C, input: Input) -> Vec<&'a str>;
}

impl<'a, C> Suggestions<'a, C> for () {
    fn suggestions(context: &C, input: Input) -> Vec<&'a str> {
        vec![]
    }
}

impl<'a, C, P> Suggestions<'a, C> for (P,)
where
    P: Suggestions<'a, C>,
{
    fn suggestions(context: &C, input: Input) -> Vec<&'a str> {
        P::suggestions(context, input)
    }
}
