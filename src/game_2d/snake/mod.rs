use crate::game::types::{Direction2D, Position2D};

pub struct Snake2D {
    head_position: Position2D,
    tail_position: Position2D,
    direction: Direction2D,
    length: u16,
    move_tail: bool,
}

impl Snake2D {
    pub fn get_length(&self) -> u16 {
        self.length
    }
    pub fn get_direction(&self) -> &Direction2D {
        &self.direction
    }
    pub fn get_tail_position(&self) -> &Position2D {
        &self.tail_position
    }
    pub fn get_head_position(&self) -> &Position2D {
        &self.head_position
    }

    pub fn get_tail_position_mut(&mut self) -> &mut Position2D {
        &mut self.tail_position
    }
    pub fn get_head_position_mut(&mut self) -> &mut Position2D {
        &mut self.head_position
    }
}
