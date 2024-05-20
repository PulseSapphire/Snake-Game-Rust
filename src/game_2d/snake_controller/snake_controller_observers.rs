use crate::game::types::{Direction2D, Position2D};

pub trait OnSnakeMove {
    fn on_event (last_position: Position2D, new_position: Position2D, direction: Direction2D, grow: bool);
}