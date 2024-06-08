use crate::game::types::direction::Direction2D;
use crate::game::types::direction::Direction2D::Stationary;
use crate::game::types::position::Position2D;

pub struct Snake2D {
    head_position: Position2D,
    tail_position: Position2D,
    direction: Direction2D,
    length: u16,
}

impl Snake2D {
    pub fn new(start_position: Position2D) -> Self {
        Snake2D {
            head_position: start_position.clone(),
            tail_position: start_position,
            direction: Stationary,
            length: 0,
        }
    }

    pub fn get_length(&self) -> u16 {
        self.length
    }
    pub fn get_direction(&self) -> &Direction2D {
        &self.direction
    }

    pub fn set_direction(&mut self, direction: Direction2D) {
        self.direction = direction;
    }
    pub fn get_tail_position(&self) -> &Position2D {
        &self.tail_position
    }

    pub fn set_tail_position(&mut self, tail_position: Position2D) {
        self.tail_position = tail_position;
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

    pub fn increment_length(&mut self) {
        self.length += 1;
    }
}
