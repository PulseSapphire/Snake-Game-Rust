use crate::game_2d::board2d::Board2D;
use crate::game_2d::snake::Snake2D;
use crate::game_2d::snake_controller::SnakeController2D;

mod board2d;
mod snake;
mod snake_controller;

pub struct Game2D<'a, const W: usize, const H: usize> {
    board: Board2D<W, H>,
    snake: Snake2D,
    snake_controller: SnakeController2D<'a, W, H>,
}
