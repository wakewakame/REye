use super::super::math::Point2d;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Key {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool, 
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Event {
    Move{ global: Point2d, local: Point2d, movement: Point2d },
    LeftDown(Key),
    RightDown(Key),
    MiddleDown(Key),
    LeftUp(Key),
    RightUp(Key),
    MiddleUp(Key),
    DblClick(Key),
    Wheel{ wheel: f64 },
    Other,
}
