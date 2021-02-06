use crate::log;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

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
pub struct Root {
    width: f64, height: f64,
    mouse: Mouse,
}

#[wasm_bindgen]
impl Root {
    pub fn new() -> Root {
        Root {
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
    pub fn mouse(&mut self, event: js_sys::Object) -> Result<(), JsValue> {
        // 引数の型チェック
        if !event.has_type::<web_sys::MouseEvent>() {
			return Err(JsValue::from_str("the argument value is not instance of MouseEvent"));
        }
        self.mouse.apply_event(event.dyn_into::<web_sys::MouseEvent>()?);
        Ok(())
    }
    pub fn keyboard(&mut self, event: js_sys::Object) -> Result<(), JsValue> {
        // 引数の型チェック
        if !event.has_type::<web_sys::KeyboardEvent>() {
			return Err(JsValue::from_str("the argument value is not instance of KeyboardEvent"));
        }
        log!("{:?}", event);
        Ok(())
    }
}
