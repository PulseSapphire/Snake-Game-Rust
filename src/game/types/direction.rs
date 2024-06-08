pub trait Direction {}

#[derive(Clone)]
pub enum Direction2D {
    Up,
    Down,
    Left,
    Right,
    Stationary,
}

impl Direction for Direction2D {}
