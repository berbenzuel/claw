pub trait HtmlElement {
    fn get_tag(&self) -> String;
    fn get_html(&self) -> String;
}