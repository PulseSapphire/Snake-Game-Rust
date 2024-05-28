pub trait Position {}

#[derive(Clone, PartialEq, Debug)]
pub struct Position2D {
    pub x: u8,
    pub y: u8,
}

impl Position for Position2D {}