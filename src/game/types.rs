#[derive(Clone)]
pub enum Direction2D {
    Up,
    Down,
    Left,
    Right,
    Stationary,
}

#[derive(Clone)]
pub struct Position2D {
    pub x: u8,
    pub y: u8,
}
