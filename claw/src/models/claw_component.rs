use crate::prelude::*;

pub trait Component {
    fn name(&self) -> String;
    fn template(&self) -> impl HtmlElement;
    fn render(&self);
}
