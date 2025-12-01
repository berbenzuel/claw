use claw::prelude::*;

pub struct Page;
impl Page {
    pub fn new() -> Self {
        Self { }
    }
}

impl Component for Page {
    fn name(&self) -> String {
       "index-page".to_string() 
    }

    fn render(&self) {
        
    }

    fn template(&self) -> Html {
        div([
            p("Rust index page")
        ])
    }
}

impl Default for Page {
    fn default() -> Self {
        Self::new()
    }
}
