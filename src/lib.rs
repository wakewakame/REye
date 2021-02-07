use wasm_bindgen::prelude::*;

mod framework;
use framework::gui;
use framework::math;

// wasmの初期化時に呼ばれる関数
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    // panicの内容をconsole.errorに出力するようにする
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    Ok(())
}

#[derive(Debug)]
struct Main {
    position: math::Point2d,
    size: math::Point2d,
    color: String,
    child: Vec<Box<dyn framework::gui::Component>>,
}
impl Main {
    fn new(x: f64, y: f64, width: f64, height: f64, color: String) -> Main {
        Main{
            position: math::Point2d::new(x, y),
            size: math::Point2d::new(width, height),
            color,
            child: Vec::new(),
        }
    }
    pub fn push(&mut self, child: Box<dyn framework::gui::Component>) {
        self.child.push(child);
    }
}
impl framework::gui::Component for Main {
    fn position(&self) -> math::Point2d { self.position }
    fn size(&self) -> math::Point2d { self.size }
    fn on_draw(&mut self, context2d: &web_sys::CanvasRenderingContext2d) {
        // 描画
        let size = self.size();
        context2d.set_fill_style(&JsValue::from_str(self.color.as_str()));
        context2d.fill_rect(0.0, 0.0, size.x, size.y);
    }
    fn child(&mut self, index: usize) -> Option<&mut Box<dyn framework::gui::Component>> { self.child.get_mut(index) }
    fn child_len(&self) -> usize { self.child.len() }
}

#[wasm_bindgen]
pub fn create_root_component() -> gui::Root {
    let mut root_component = gui::Root::new();

    let mut c1 = Main::new(100.0, 100.0, 160.0, 200.0, "#F0F".into());
    let c1_1 = Main::new(10.0, 10.0, 30.0, 30.0, "#000".into());
    let c1_2 = Main::new(40.0, 20.0, 20.0, 30.0, "#F00".into());

    c1.push(Box::new(c1_1));
    c1.push(Box::new(c1_2));

    let c2 = Main::new(300.0, 120.0, 100.0, 100.0, "#0FF".into());

    root_component.push(Box::new(c1));
    root_component.push(Box::new(c2));

    return root_component;
}
