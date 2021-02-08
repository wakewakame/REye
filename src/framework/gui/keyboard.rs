#[derive(Debug, Clone, PartialEq)]
pub struct Key {
    pub key: String,
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool, 
}

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    Up(Key),
    Down(Key),
    Press(Key),
    Other,
}
