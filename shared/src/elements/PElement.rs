use crate::models::HtmlElement;

pub struct P {
    pub content: String
}

impl HtmlElement for P {
    fn get_tag(&self) -> String {
        "p".to_string()
    }

    fn get_html(&self) -> String {
        format!("<{}>{}</{}>", self.get_tag(), self.content, self.get_tag())
    }
}