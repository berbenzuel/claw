use wasm_bindgen::prelude::*;
use shared::*;

#[wasm_bindgen]
pub fn get_app() -> String {
    P::new("Hello from Rust p element").get_html()
}
