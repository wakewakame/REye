use wasm_bindgen::prelude::*;

mod framework;
use framework::gui;

// wasmの初期化時に呼ばれる関数
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    // panicの内容をconsole.errorに出力するようにする
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    Ok(())
}

#[wasm_bindgen]
pub fn create_root_component() -> gui::Root {
    let root_component = gui::Root::new();
    return root_component;
}
