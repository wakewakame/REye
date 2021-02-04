extern crate console_error_panic_hook;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// println!の代わりにlog!でコンソール出力できるようにする
#[allow(unused_macros)]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

// wasmの初期化時に呼ばれる関数
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    // panicの内容をconsole.errorに出力するようにする
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    Ok(())
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct RootComponent {
    width: f64, height: f64,
}

#[wasm_bindgen]
impl RootComponent {
    pub fn new() -> RootComponent {
        RootComponent { width: 0.0, height: 0.0 }
    }
    pub fn draw(&mut self, context2d: js_sys::Object) -> Result<(), JsValue> {
        // 引数の型チェック
        if let Ok(context2d) = context2d.dyn_into::<web_sys::CanvasRenderingContext2d>() {
            // 描画
            context2d.set_fill_style(&JsValue::from_str("#f0f"));
            context2d.fill_rect(10.0, 10.0, 100.0, 100.0);
        }
        else {
			return Err(JsValue::from_str("the argument value is not instance of CanvasRenderingContext2d"));
        }

        Ok(())
    }
    pub fn resize(&mut self, width: f64, height: f64) {
        self.width = width;
        self.height = height;
    }
}
