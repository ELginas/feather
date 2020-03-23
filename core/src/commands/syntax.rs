pub trait Syntax<C> {
    fn syntax(context: &C) -> &str;
}
