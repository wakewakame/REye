use crate::log;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use super::super::math::Point2d;
use super::{ Component, CompRc };
use super::mouse;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Root {
    size: Point2d,
    children: Vec<CompRc>,
}

impl Root {
    pub fn new() -> Root {
        Root {
            size: Point2d::new(0.0, 0.0),
            children: Vec::new(),
        }
    }
    pub fn push(&mut self, child: CompRc) {
        self.children.push(child);
    }
}

impl Component for Root {
    fn position(&self) -> Point2d { Point2d::new(0.0, 0.0) }
    fn size(&self) -> Point2d { self.size }
    fn children_rc(&self) -> Vec<CompRc> { self.children.clone() }
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
        let event = event.dyn_into::<web_sys::MouseEvent>()?;

        // マウス座標の取得
        let global = Point2d::new(event.offset_x() as f64, event.offset_y() as f64);
        let mut local = global.clone();
        let movement = Point2d::new(event.movement_x() as f64, event.movement_y() as f64);

        // マウス直下のコンポーネントを取得
        let hit_components = self.get_hit_component(local);

        // コンポーネントのローカル座標基準でのマウス座標を計算
        for comp in hit_components.iter() {
            local -= comp.borrow().position();
        }

        // イベントと同時に押されている特殊キーの取得
        let key = mouse::Key {
            shift: event.shift_key(),
            ctrl: event.ctrl_key(),
            alt: event.alt_key(),
        };
        
        let event: mouse::Event = match event.type_().as_str() {
            "mousemove" => mouse::Event::Move { global, local, movement },
            "mousedown" => match event.which() {
                1 => mouse::Event::LeftDown(key),
                2 => mouse::Event::MiddleDown(key),
                3 => mouse::Event::RightDown(key),
                _ => mouse::Event::Other,
            },
            "mouseup" => match event.which() {
                1 => mouse::Event::LeftUp(key),
                2 => mouse::Event::MiddleUp(key),
                3 => mouse::Event::RightUp(key),
                _ => mouse::Event::Other,
            },
            "dblclick" => mouse::Event::DblClick(key),
            _ => mouse::Event::Other
        };

        if event == mouse::Event::Other { return Ok(()); }

        if let Some(comp) = hit_components.first() {
            comp.borrow_mut().on_mouse(event);
        }

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
