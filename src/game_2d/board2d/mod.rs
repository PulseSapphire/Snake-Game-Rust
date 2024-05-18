use crate::game::types::Position2D;
use crate::game_2d::board2d::layout::Layout2D;
use crate::game_2d::snake::Snake2D;

mod layout;

pub struct Board2D<const W: u8, const H: u8> {
    snake: Snake2D,
    layout: Layout2D<W, H>,
    food: Position2D,
}

impl <const W: u8, const H: u8> Board2D<W, H> {
    pub fn new (snake: Snake2D, food: Position2D) -> Self {
        Self {
            snake,
            food,
            layout: Layout2D::<W, H>::new(),
        }
    }
}