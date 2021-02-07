use crate::log;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use super::super::math;

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
    child: Vec<Box<dyn super::Component>>,
}

impl Root {
    pub fn new() -> Root {
        Root {
            width: 0.0, height: 0.0,
            mouse: Mouse::new(),
            child: Vec::new()
        }
    }
    pub fn push(&mut self, child: Box<dyn super::Component>) {
        self.child.push(child);
    }
    fn draw_child(component: &mut Box<dyn super::Component>, context2d: &web_sys::CanvasRenderingContext2d) {
        let position = component.position();
        context2d.save();
        let _ = context2d.translate(position.x, position.y);
        component.on_draw(context2d);
        for index in 0..component.child_len() {
            match component.child(index) {
                Some(c) => { Self::draw_child(c, context2d); },
                None => (),
            }
        }
        context2d.restore();
    }
}

impl super::Component for Root {
    fn position(&self) -> math::Point2d { math::Point2d::new(0.0, 0.0) }
    fn size(&self) -> math::Point2d { math::Point2d::new(self.width, self.height) }
    fn child(&mut self, index: usize) -> Option<&mut Box<dyn super::Component>> { self.child.get_mut(index) }
    fn child_len(&self) -> usize { self.child.len() }
}

#[wasm_bindgen]
impl Root {
    pub fn draw(&mut self, context2d: js_sys::Object) -> Result<(), JsValue> {
        // 引数の型チェック
        if !context2d.has_type::<web_sys::CanvasRenderingContext2d>() {
			return Err(JsValue::from_str("the argument value is not instance of CanvasRenderingContext2d"));
        }
        let context2d = context2d.dyn_into::<web_sys::CanvasRenderingContext2d>()?;

        for c in &mut self.child {
            Self::draw_child(c, &context2d); 
        }

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
