use wasm_bindgen::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;

mod framework;
use framework::gui;
use framework::math::Point2d;
use framework::gui::{ Component, CompRc, Context2d, mouse };

// wasmの初期化時に呼ばれる関数
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    // panicの内容をconsole.errorに出力するようにする
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    Ok(())
}

#[derive(Debug)]
struct Main {
    position: Point2d,
    size: Point2d,
    color: String,
    children: Vec<CompRc>,
    flag: bool,
}
impl Main {
    fn create(x: f64, y: f64, width: f64, height: f64, color: String) -> CompRc<Main> {
        let comp = Main{
            position: Point2d::new(x, y),
            size: Point2d::new(width, height),
            color,
            children: Vec::new(),
            flag: false,
        };
        Rc::new(RefCell::new(comp))
    }
    fn push<T: Component + 'static>(&mut self, child: CompRc<T>) {
        self.children.push(child);
    }
}
impl Component for Main {
    fn position(&self) -> Point2d { self.position }
    fn size(&self) -> Point2d { self.size }
    fn on_draw(&mut self, ctx: &Context2d) {
        // 描画
        let size = self.size();
        ctx.set_line_width(1.0);
        ctx.set_stroke_style(&JsValue::from_str("#000"));
        ctx.set_fill_style(&JsValue::from_str(self.color.as_str()));
        if self.flag {
            ctx.set_fill_style(&JsValue::from_str("#fff"));
        }
        ctx.begin_path();
        ctx.rect(0.0, 0.0, size.x, size.y);
        ctx.fill();
        ctx.stroke();
    }
    fn children_rc(&self) -> Vec<CompRc> { self.children.clone() }
    fn on_mouse(&mut self, event: mouse::Event) {
        match event {
            mouse::Event::LeftDown(_) => {
                self.flag = true;
            },
            mouse::Event::LeftUp(_) => {
                self.flag = false;
            },
            _ => ()
        }
    }
}

#[wasm_bindgen]
pub fn create_root_component() -> gui::Root {
    let mut root_component = gui::Root::new();

    let c1 = Main::create(100.0, 100.0, 160.0, 200.0, "#F0F".into());
    let c1_1 = Main::create(10.0, 10.0, 30.0, 30.0, "#000".into());
    let c1_2 = Main::create(40.0, 20.0, 20.0, 30.0, "#F00".into());

    c1.borrow_mut().push(c1_1);
    c1.borrow_mut().push(c1_2);

    let c2 = Main::create(300.0, 120.0, 100.0, 100.0, "#0FF".into());

    root_component.push(c1);
    root_component.push(c2);

    return root_component;
}
