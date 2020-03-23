pub trait Help<C> {
    fn help(context: &C) -> &str;
}
