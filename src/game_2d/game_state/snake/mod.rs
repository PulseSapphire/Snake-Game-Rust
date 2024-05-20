use crate::game::types::{Direction2D, Position2D};
use crate::game::types::Direction2D::Stationary;

pub struct Snake2D {
    head_position: Position2D,
    tail_position: Position2D,
    direction: Direction2D,
    length: u16,
    move_tail: bool,
}

impl Snake2D {

    pub fn new (start_position: Position2D) -> Self {
        Snake2D {
            head_position: start_position.clone(),
            tail_position: start_position,
            direction: Stationary,
            length: 0,
            move_tail: false,
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

    pub fn get_move_tail(&self) -> bool {
        self.move_tail
    }
    pub fn set_move_tail(&mut self, move_tail: bool) {
        self.move_tail = move_tail;
    }
    pub fn increment_length(&mut self) {
        self.length += 1;
    }
}
