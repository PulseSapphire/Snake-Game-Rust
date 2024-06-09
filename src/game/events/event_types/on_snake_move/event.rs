use crate::game::events::Event;
use crate::game::types::position::Position;

pub struct SnakeMoveEvent<'a, P: Position + 'a> {
    last_head_position: &'a P,
    new_head_position: &'a P,
    last_tail_position: &'a P,
    current_tail_position: &'a P,
    length: u16
}

impl<'a, P: Position + 'a> SnakeMoveEvent<'a, P> {
    pub fn new (
        last_head_position: &'a P,
        new_head_position: &'a P,
        last_tail_position: &'a P,
        current_tail_position: &'a P,
        length: u16
    ) -> Self {
        Self {
            last_head_position,
            new_head_position,
            last_tail_position,
            current_tail_position,
            length,
        }
    }

    pub fn get_last_head_position(&self) -> &'a P {
        self.last_head_position
    }
    pub fn get_new_head_position(&self) -> &'a P {
        self.new_head_position
    }
    pub fn get_last_tail_position(&self) -> &'a P {
        self.last_tail_position
    }
    pub fn get_current_tail_position(&self) -> &'a P {
        self.current_tail_position
    }
    pub fn get_length(&self) -> u16 {
        self.length
    }
}


impl<'a, P: Position> Event for SnakeMoveEvent<'a, P> {}
