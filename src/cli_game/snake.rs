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
    position: Position,
    direction: Direction,
    length: u16,

}

impl Snake {
    pub fn position(&self) -> &Position {
        &self.position
    }

    pub fn set_position(&mut self, position: Position) {
        self.position = position;
    }
}