#[cfg(debug_assertions)]
extern crate console_error_panic_hook;
#[cfg(debug_assertions)]
use std::panic;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let val = document.create_element("p")?;
    val.set_inner_html("Hello from Rust!");

    body.append_child(&val)?;

    Ok(())
}

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}
