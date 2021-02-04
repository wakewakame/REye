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

#[derive(Debug)]
struct Mouse {
    x: f64, y: f64,
}

impl Mouse {
    fn new() -> Mouse {
        Mouse { x: 0.0, y: 0.0 }
    }
    fn apply_event(&mut self, event: web_sys::MouseEvent) {
        self.x = event.offset_x() as f64;
        self.y = event.offset_y() as f64;
    }
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct RootComponent {
    width: f64, height: f64,
    mouse: Mouse,
}

#[wasm_bindgen]
impl RootComponent {
    pub fn new() -> RootComponent {
        RootComponent {
            width: 0.0, height: 0.0,
            mouse: Mouse::new()
        }
    }
    pub fn draw(&mut self, context2d: js_sys::Object) -> Result<(), JsValue> {
        // 引数の型チェック
        if !context2d.has_type::<web_sys::CanvasRenderingContext2d>() {
			return Err(JsValue::from_str("the argument value is not instance of CanvasRenderingContext2d"));
        }
        let context2d = context2d.dyn_into::<web_sys::CanvasRenderingContext2d>()?;

        // 描画
        context2d.set_fill_style(&JsValue::from_str("#fff"));
        context2d.fill_rect(0.0, 0.0, self.width, self.height);
        context2d.set_fill_style(&JsValue::from_str("#f0f"));
        context2d.fill_rect(self.mouse.x, self.mouse.y, 100.0, 100.0);

        Ok(())
    }
    pub fn resize(&mut self, width: f64, height: f64) {
        self.width = width;
        self.height = height;
    }
    pub fn dispatch_event(&mut self, event: js_sys::Object) -> Result<(), JsValue> {
        // 引数の型チェック
        if !event.has_type::<web_sys::Event>() {
			return Err(JsValue::from_str("the argument value is not instance of Event"));
        }

        if let Ok(e) = event.dyn_into::<web_sys::MouseEvent>() { self.mouse(e); }

        Ok(())
    }
    fn mouse(&mut self, e: web_sys::MouseEvent) {
        self.mouse.apply_event(e);
    }
}
