pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    Stationary,
}

pub struct Position {
    pub x: u8,
    pub y: u8,
}

pub struct Snake {
    head_position: Position,
    tail_position: Position,
    direction: Direction,
    length: u16,
}

impl Snake {
}