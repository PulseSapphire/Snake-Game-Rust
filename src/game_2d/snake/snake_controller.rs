use std::ops::Deref;
use crate::game::types::{Direction2D, Position2D};
use crate::game_2d::board2d::Board2D;
use crate::game_2d::snake::Snake2D;

pub struct SnakeController2D<'a, const W: usize, const H: usize> {
    snake: &'a mut Snake2D,
    board: &'a mut Board2D<W, H>,
}

impl <'a, const W: usize, const H: usize> SnakeController2D<'a, W, H> {
    pub fn new(snake2d: &'a mut Snake2D, board2d: &'a mut Board2D<W, H>) -> Self {
        Self {
            snake: snake2d,
            board: board2d
        }
    }

    fn move_head (&mut self) -> Result<(), &'static str> {
        let dir = self.snake.get_direction().clone();
        let Position2D { x: ref mut hx, y: ref mut hy } = self.snake.get_head_position_mut();

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
                if *hx == self.board.get_width() - 1 {
                    return Err("Snake hits the right wall!");
                }

                *hx += 1;
            }

            Stationary => (),
        }

        Ok(())
    }

    fn move_tail (&mut self) {
        let current_tail_pos = &self.snake.tail_position;
        let current_val = self.board.get_layout().get_val_at_pos(current_tail_pos);

        if let Some(position) = self.board.get_layout().get_adjacent_position_with_val(current_tail_pos, current_val + 1) {
            self.snake.tail_position = position;
        }
        else {
            panic!("Could not find new valid tail position for snake.");
        }


    }

    pub fn move_snake(&mut self) -> Result<(), &'static str> {
        self.move_head()?;
        self.move_tail();
        Ok(())
    }
}