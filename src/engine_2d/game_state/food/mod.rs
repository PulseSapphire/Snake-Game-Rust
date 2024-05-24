use crate::game::types::Position2D;

pub struct Food {
    position: Position2D
}

impl Food {
    pub fn new (start_position: Position2D) -> Self {
        Self {
            position: start_position
        }
    }
    pub fn get_position (&self) -> &Position2D {
        &self.position
    }
}