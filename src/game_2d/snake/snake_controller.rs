use crate::game::types::Direction2D;
use crate::game_2d::board2d::Board2D;
use crate::game_2d::snake::Snake2D;

pub struct SnakeController2D {
    snake: Snake2D,
    board: Board2D,
}

impl SnakeController2D {
    pub fn new(snake2d: Snake2D, board2d: Board2D) -> Self {
        Self {
            snake: snake2d,
            board: board2d
        }
    }
}