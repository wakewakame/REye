use super::super::math;

pub trait Component: std::fmt::Debug {
    fn position(&self) -> math::Point2d { math::Point2d::new(0.0, 0.0) }
    fn size(&self) -> math::Point2d { math::Point2d::new(0.0, 0.0) }

    fn on_draw(&mut self, _: &web_sys::CanvasRenderingContext2d) {}
    
    fn is_hit(&self, position: math::Point2d) -> bool {
        let size = self.size();
        if position.x <= 0.0 { return false; }
        if size.x < position.x { return false; }
        if position.y <= 0.0 { return false; }
        if size.y < position.y { return false; }
        return true;
    }

    fn child(&mut self, _: usize) -> Option<&mut Box<dyn Component>> { None }
    fn child_len(&self) -> usize { 0 }
}
