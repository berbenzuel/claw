pub trait HtmlElement {
    fn get_tag(&self) -> &str;
    fn get_html(&self) -> String;
}

pub fn view<T: HtmlElement>(element: &T) -> String {
    element.get_html()
}
