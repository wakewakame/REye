use crate::log;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::rc::Rc;

use super::super::math::Point2d;
use super::{ Component, CompRc, CompWeak };

/*
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
*/

#[wasm_bindgen]
#[derive(Debug)]
pub struct Root {
    size: Point2d,
    child: Vec<CompRc>,
}

impl Root {
    pub fn new() -> Root {
        Root {
            size: Point2d::new(0.0, 0.0),
            child: Vec::new(),
        }
    }
    pub fn push(&mut self, child: CompRc) {
        self.child.push(child);
    }
}

impl Component for Root {
    fn position(&self) -> Point2d { Point2d::new(0.0, 0.0) }
    fn size(&self) -> Point2d { self.size }
    fn child(&self) -> Vec<CompWeak> {
        self.child.iter().map(|comp| Rc::downgrade(&comp)).collect()
    }
}

#[wasm_bindgen]
impl Root {
    pub fn animation(&mut self, context2d: js_sys::Object) -> Result<(), JsValue> {
        // 引数の型チェック
        if !context2d.has_type::<web_sys::CanvasRenderingContext2d>() {
			return Err(JsValue::from_str("the argument value is not instance of CanvasRenderingContext2d"));
        }
        let context2d = context2d.dyn_into::<web_sys::CanvasRenderingContext2d>()?;
        self.draw(&context2d);
        Ok(())
    }
    pub fn resize(&mut self, width: f64, height: f64) {
        self.size.x = width;
        self.size.y = height;
    }
    pub fn mouse(&mut self, event: js_sys::Object) -> Result<(), JsValue> {
        // 引数の型チェック
        if !event.has_type::<web_sys::MouseEvent>() {
			return Err(JsValue::from_str("the argument value is not instance of MouseEvent"));
        }
        //self.mouse.apply_event(event.dyn_into::<web_sys::MouseEvent>()?);
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
