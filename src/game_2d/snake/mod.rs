use crate::game::types::{Direction2D, Position2D};

mod snake_controller;

pub struct Snake2D {
    head_position: Position2D,
    tail_position: Position2D,
    direction: Direction2D,
    length: u16,
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
}