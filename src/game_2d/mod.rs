use crate::game::types::Position2D;
use crate::game_2d::board2d::Board2D;
use crate::game_2d::snake::Snake2D;
use crate::game_2d::snake_controller::SnakeController2D;

mod board2d;
mod snake;
mod snake_controller;

pub struct Game2D<const W: usize, const H: usize> {
    snake_controller: SnakeController2D<W, H>,
}

impl <const W: usize, const H: usize> Game2D<W, H> {

    const U8_MAX_IN_USIZE: usize = u8::MAX as usize;
    pub fn new (start_position: Position2D, starting_food_position: Position2D) -> Self {
        if W > Self::U8_MAX_IN_USIZE || H > Self::U8_MAX_IN_USIZE {
            panic!("Each dimension of the game can have a max height and width of 255 only.");
        }

        Self {
            snake_controller: SnakeController2D::new(Snake2D::new(start_position), Board2D::new(starting_food_position)),
        }
    }
}