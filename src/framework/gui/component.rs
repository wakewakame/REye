use std::rc::{ Rc, Weak };
use std::cell::RefCell;

use super::super::math::Point2d;

pub type CompRc<T = dyn Component> = Rc<RefCell<T>>;
pub type CompWeak<T = dyn Component> = Weak<RefCell<T>>;
pub type Context2d = web_sys::CanvasRenderingContext2d;

pub trait Component: std::fmt::Debug {
    // スタイル
    fn position(&self) -> Point2d { Point2d::new(0.0, 0.0) }
    fn size(&self) -> Point2d { Point2d::new(0.0, 0.0) }

    // 子コンポーネント
    fn child(&self) -> Vec<CompWeak>;

    // イベント
    fn on_draw(&mut self, _: &Context2d) {}
    
    // ユーティリティ関数
    fn draw(&mut self, ctx: &Context2d) {
        ctx.save();
        let pos = self.position();
        let _ = ctx.translate(pos.x, pos.y);
        self.on_draw(ctx);
        let child = self.child();
        for c in child {
            if let Some(c) = c.upgrade() {
                c.borrow_mut().draw(ctx);
            }
        }
        ctx.restore();
    }
    fn is_hit(&self, position: Point2d) -> bool {
        let size = self.size();
        if position.x <= 0.0 { return false; }
        if size.x < position.x { return false; }
        if position.y <= 0.0 { return false; }
        if size.y < position.y { return false; }
        return true;
    }
}
