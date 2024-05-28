use crate::game::types::position::Position2D;

pub struct Food {
    position: Position2D,
}

impl Food {
    pub fn new(start_position: Position2D) -> Self {
        Self {
            position: start_position,
        }
    }
    pub fn get_position(&self) -> &Position2D {
        &self.position
    }

    pub fn set_position(&mut self, position: Position2D) {
        self.position = position;
    }

    pub fn get_position_mut(&mut self) -> &mut Position2D {
        &mut self.position
    }
}
