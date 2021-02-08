use std::rc::{ Rc, Weak };
use std::cell::RefCell;

use super::super::math::Point2d;
use super::mouse;

pub type CompRc<T = dyn Component> = Rc<RefCell<T>>;
pub type CompWeak<T = dyn Component> = Weak<RefCell<T>>;
pub type Context2d = web_sys::CanvasRenderingContext2d;

pub trait Component: std::fmt::Debug {
    // スタイル
    fn position(&self) -> Point2d { Point2d::new(0.0, 0.0) }
    fn set_position(&mut self, position: Point2d);
    fn size(&self) -> Point2d { Point2d::new(0.0, 0.0) }
    fn set_size(&mut self, size: Point2d);

    // 子コンポーネント
    fn children_rc(&self) -> Vec<CompRc>;

    // イベント
    fn on_update(&mut self) {}
    fn on_after_update(&mut self) {}
    fn on_draw(&mut self, _: &Context2d) {}
    fn on_after_draw(&mut self, _: &Context2d) {}
    fn on_mouse(&mut self, _: mouse::Event) {}
    
    // ユーティリティ関数
    fn update(&mut self) {
        self.on_update();
        let children = self.children_rc();
        for child in children {
            child.borrow_mut().update();
        }
        self.on_after_update();
    }
    fn draw(&mut self, ctx: &Context2d) {
        ctx.save();
        let pos = self.position();
        let _ = ctx.translate(pos.x, pos.y);
        self.on_draw(ctx);
        let children = self.children_rc();
        for child in children {
            child.borrow_mut().draw(ctx);
        }
        self.on_after_draw(ctx);
        ctx.restore();
    }
    fn children(&self) -> Vec<CompWeak> {
        self.children_rc().iter().map(|comp| Rc::downgrade(comp)).collect()
    }
    fn is_hit(&self, position: Point2d) -> bool {
        let size = self.size();
        if position.x < 0.0 { return false; }
        if size.x <= position.x { return false; }
        if position.y < 0.0 { return false; }
        if size.y <= position.y { return false; }
        return true;
    }
    fn get_hit_component(&self, position: Point2d) -> Vec<CompWeak> {
        for child in self.children_rc() {
            let child_ref = child.borrow();
            let position = position - child_ref.position();
            if child_ref.is_hit(position) {
                let mut comp = child_ref.get_hit_component(position);
                comp.push(Rc::downgrade(&child));
                return comp;
            }
        } 
        return Vec::new();
    }
}
