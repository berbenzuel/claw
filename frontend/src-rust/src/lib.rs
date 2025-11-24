use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {name} from rust!")
}

#[wasm_bindgen]
pub fn get_app() -> String {
    "<p>Hello from Rust</p>".to_string()
}
