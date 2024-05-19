use crate::game::types::{Direction2D, Position2D};
use crate::game_2d::board2d::Board2D;
use crate::game_2d::snake::Snake2D;

pub struct SnakeController2D<'a, const W: usize, const H: usize> {
    snake: Snake2D,
    board: &'a Board2D<W, H>,
}

impl <'a, const W: usize, const H: usize> SnakeController2D<'a, W, H> {
    pub fn new(snake2d: Snake2D, board2d: &'a Board2D<W, H>) -> Self {
        Self {
            snake: snake2d,
            board: board2d
        }
    }

    pub fn move_snake(&mut self) -> Result<(), &'static str> {
        let dir = self.snake.get_direction();
        let Position2D { x: ref mut hx, y: ref mut hy } = self.snake.get_head_position();
        let Position2D { x: ref mut tx, y: ref mut ty } = self.snake.get_tail_position();

        use Direction2D::*;
        match dir {
            Up => {
                if *hy == 0 {
                    return Err("Snake hits the top wall!");
                }

                *hy -= 1;
            }

            Down => {
                if *hy == self.board.get_height() - 1 {
                    return Err("Snake hits the bottom wall!");
                }

                *hy += 1;
            }

            Left => {
                if *hx == 0 {
                    return Err("Snake hits the left wall!");
                }
                *hx -= 1;
            }

            Right => {
                if (*hx == self.board.get_width() - 1) {
                    return Err("Snake hits the right wall!");
                }

                *hx += 1;
            }

            Stationary => (),
        }

        Ok(())
    }
}