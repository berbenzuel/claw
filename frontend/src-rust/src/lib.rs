use claw::{core::ClawApp};
use wasm_bindgen::prelude::*;
pub mod app;

#[wasm_bindgen]
pub fn init_app() {
    let app = ClawApp::new();
    let page = app::page::Page;

    app.attach(&page).expect("Cannot attach Wasm to Vite client");
}
