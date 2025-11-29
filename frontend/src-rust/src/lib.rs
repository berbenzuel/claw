use claw::{core::ClawApp, prelude::*};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn init_app() {
    let app = ClawApp::new();

    struct TestCmp;
    impl Component for TestCmp {
        fn name(&self) -> String {
            "test".to_string()
        }

        fn template(&self) -> impl HtmlElement {
            div([
                p("Rust test component")
            ])
        }

        fn render(&self) {

        }
    }

    let test = TestCmp {};

    app.attach(&test).expect("Cannot attach Wasm to Vite client");
}
