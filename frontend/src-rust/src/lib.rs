use shared::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn get_app() -> String {
    view(
        div([
            p("Hello from rust wasm"),
            p("Hi from rust wasm again"),
            p("Hi from rust wasm for the third time!")
        ])
    )
}
