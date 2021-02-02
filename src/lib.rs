// デバッグビルド時にpanicの内容をconsole.errorに出力するようにする
#[cfg(debug_assertions)]
extern crate console_error_panic_hook;
#[cfg(debug_assertions)]
use std::panic;

use wasm_bindgen::prelude::*;

// wasmの初期化時に呼ばれる関数
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    // デバッグビルド時にpanicの内容をconsole.errorに出力するようにする
    #[cfg(debug_assertions)]
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    Ok(())
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct RootComponent {
    i: i32,
}

#[wasm_bindgen]
impl RootComponent {
    pub fn new() -> RootComponent {
        RootComponent { i: 0 }
    }
    pub fn add(&mut self) {
        self.i += 1;
    }
    pub fn get(&self) -> i32 {
        self.i
    }
}
