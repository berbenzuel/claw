pub trait HTMLElement {
    fn get_tag(&self) -> String;
    fn get_html(&self) -> String;
}

pub struct P {
    pub content: String
}

impl P {
    pub fn new(content: &str) -> P {
        P { content: content.to_string() }
    }
}

impl HTMLElement for P {
    fn get_tag(&self) -> String {
        "p".to_string()
    }

    fn get_html(&self) -> String {
        format!("<{}>{}</{}>", self.get_tag(), self.content, self.get_tag())
    }
}
