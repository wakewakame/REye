use std::rc::{ Rc, Weak };
use std::cell::RefCell;

use super::super::math::Point2d;
use super::{ mouse, keyboard };

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
    fn on_keyboard(&mut self, _: keyboard::Event) {}
    
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
        ctx.begin_path();
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

        if (0.0 <= position.x) && (position.x < size.x) &&
           (0.0 <= position.y) && (position.y < size.y) { return true; } 
        
        for child in self.children_rc().iter() {
            let child = child.borrow();
            if child.is_hit(position - child.position()) {
                return true;
            }
        }

        return false;
    }
    fn get_hit_component(&self, position: Point2d) -> Vec<CompWeak> {
        // 指定された座標の一番手前に映るコンポーネントを取得する
        // 戻り値は当たり判定で得たコンポーネントを最初の要素として
        // 1つずつ親コンポーネントを辿る配列を返す
        let mut children = self.children_rc().clone();
        children.reverse();
        for child in children {
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
