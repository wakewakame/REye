pub trait Component: std::fmt::Debug {
    fn on_setup(&mut self) {}
    fn on_update(&mut self) {}
    fn on_after_update(&mut self) {}
    fn on_draw(&mut self, _: js_sys::Object) {}
    fn on_after_draw(&mut self, _: js_sys::Object) {}
    fn add_child(&mut self, child: Box<dyn Component>);
}
