use crate::game_2d::board2d::Board2D;
use crate::game_2d::snake::Snake2D;
use crate::game_2d::snake_controller::SnakeController2D;

mod board2d;
mod snake;
mod snake_controller;

pub struct Game2D<const W: usize, const H: usize> {
    snake_controller: SnakeController2D<W, H>,
}
