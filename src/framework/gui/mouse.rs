use super::super::math::Point2d;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Key {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool, 
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Button {
    Left(Key),
    Right(Key),
    Middle(Key),
    Other
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Event {
    Move{ global: Point2d, local: Point2d, movement: Point2d },
    Down(Button),
    Up(Button),
    DblClick(Button),
    Wheel{ wheel: f64 },
    Other,
}
