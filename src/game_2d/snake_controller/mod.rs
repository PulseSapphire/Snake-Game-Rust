mod snake_controller_observers;

use crate::game::types::{Direction2D, Position2D};
use crate::game_2d::board2d::Board2D;
use crate::game_2d::snake::Snake2D;

pub struct SnakeController2D<const W: usize, const H: usize> {
    snake: Snake2D,
    board: Board2D<W, H>,
}

impl<const W: usize, const H: usize> SnakeController2D<W, H> {
    pub fn new(snake2d: Snake2D, board2d: Board2D<W, H>) -> Self {
        Self {
            snake: snake2d,
            board: board2d,
        }
    }

    fn move_head(&mut self) -> Result<(), &'static str> {
        let dir = self.snake.get_direction().clone();
        let Position2D {
            x: ref mut hx,
            y: ref mut hy,
        } = self.snake.get_head_position_mut();

        let prev_hx = hx.clone();
        let prev_hy = hy.clone();

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

        let layout = self.board.get_layout();

        let prev_head_val = layout.get_val_at_index(prev_hx as usize, prev_hy as usize);
        let hx = *hx as usize;
        let hy = *hy as usize;
        let new_head_location = self.board.get_layout().get_val_at_index(hx, hy);

        if new_head_location > 0 {
            return Err("Snake hits itself!");
        }

        self.board
            .get_layout_mut()
            .set_val_at_index(hx, hy, prev_head_val);

        Ok(())
    }

    fn move_tail(&mut self) {
        let current_tail_pos = &self.snake.get_tail_position();
        let current_val = self.board.get_layout().get_val_at_pos(current_tail_pos);

        self.board.get_layout_mut().set_val_at_index(
            current_tail_pos.x as usize,
            current_tail_pos.y as usize,
            0,
        );

        if let Some(position) = self
            .board
            .get_layout()
            .get_adjacent_position_with_val(current_tail_pos, current_val + 1)
        {
            self.snake.set_tail_position(position);
        } else {
            panic!("Could not find new valid tail position for snake.");
        }
    }

    pub fn move_snake(&mut self) -> Result<(), &'static str> {
        if let Direction2D::Stationary = self.snake.get_direction() {
            return Ok(());
        }

        self.move_head()?;

        let can_move_tail = self.snake.get_move_tail();
        if can_move_tail {
            self.move_tail();
        }

        if can_move_tail {
            self.snake.set_move_tail(true);
            self.snake.increment_length();
        }

        Ok(())
    }
}
