use crate::log;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use super::super::math::Point2d;
use super::{ Component, CompRc, CompWeak };
use super::mouse;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Root {
    // コンポーネントスタイル
    position: Point2d,
    size: Point2d,

    // 子コンポーネント
    children: Vec<CompRc>,

    // ドラッグ中のコンポーネント(左クリック, 中央クリック, 右クリック)
    drag: (Vec<CompWeak>, Vec<CompWeak>, Vec<CompWeak>),
}

impl Root {
    pub fn new() -> Root {
        Root {
            position: Point2d::new(0.0, 0.0),
            size: Point2d::new(0.0, 0.0),
            children: Vec::new(),
            drag: (Vec::new(), Vec::new(), Vec::new()),
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

    fn on_update(&mut self) {
        for child in self.children.iter() {
            child.borrow_mut().set_position(self.position);
            child.borrow_mut().set_size(self.size);
        }
    }

    fn set_position(&mut self, _: Point2d) {}
    fn set_size(&mut self, _: Point2d) {}
}

#[wasm_bindgen]
impl Root {
    pub fn animation(&mut self, context2d: js_sys::Object) -> Result<(), JsValue> {
        // 引数の型チェック
        if !context2d.has_type::<web_sys::CanvasRenderingContext2d>() {
			return Err(JsValue::from_str("the argument value is not instance of CanvasRenderingContext2d"));
        }
        let context2d = context2d.dyn_into::<web_sys::CanvasRenderingContext2d>()?;
        self.update();
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

        // どのボタンに対するイベントかを取得
        let button_num = event.which();

        // ドラッグ中のコンポーネントがあればそれを求める
        let mut target_component = match button_num {
            1 => self.drag.0.clone(),
            2 => self.drag.1.clone(),
            3 => self.drag.2.clone(),
            _ => Vec::new(),
        };

        // ドラッグ中のコンポーネントがない場合はマウス直下のコンポーネントを取得
        if target_component.is_empty() {
            target_component = self.get_hit_component(local);
        }

        // コンポーネントのローカル座標基準でのマウス座標を計算
        for comp in target_component.iter() {
            if let Some(comp) = comp.upgrade() {
                local -= comp.borrow().position();
            }
            else {
                return Err(JsValue::from_str("failed to send mouse event"));
            }
        }

        // イベントと同時に押されている特殊キーの取得
        let key = mouse::Key {
            shift: event.shift_key(),
            ctrl: event.ctrl_key(),
            alt: event.alt_key(),
        };
        
        // イベントの作成
        let event: mouse::Event = match event.type_().as_str() {
            "mousemove" => mouse::Event::Move { global, local, movement },
            "mousedown" => match button_num {
                1 => { self.drag.0 = target_component.clone(); mouse::Event::LeftDown(key) },
                2 => { self.drag.1 = target_component.clone(); mouse::Event::MiddleDown(key) },
                3 => { self.drag.2 = target_component.clone(); mouse::Event::RightDown(key) },
                _ => mouse::Event::Other,
            },
            "mouseup" => match button_num {
                1 => { self.drag.0 = Vec::new(); mouse::Event::LeftUp(key) },
                2 => { self.drag.1 = Vec::new(); mouse::Event::MiddleUp(key) },
                3 => { self.drag.2 = Vec::new(); mouse::Event::RightUp(key) },
                _ => mouse::Event::Other,
            },
            "dblclick" => mouse::Event::DblClick(key),
            _ => mouse::Event::Other
        };

        // 知らないイベントだった場合は処理を終了
        if event == mouse::Event::Other { return Ok(()); }

        // イベントを送信
        if let Some(comp) = target_component.first() {
            if let Some(comp) = comp.upgrade() {
                comp.borrow_mut().on_mouse(event);
            }
            else {
                return Err(JsValue::from_str("failed to send mouse event"));
            }
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
