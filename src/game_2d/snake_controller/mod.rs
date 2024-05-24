pub mod snake_controller_observers;

use std::cell::RefCell;
use std::rc::Weak;
use crate::game::types::{Direction2D, Position2D};
use crate::game_2d::game_state::board2d::Board2D;
use crate::game_2d::game_state::GameState;
use crate::game_2d::game_state::snake::Snake2D;

use crate::game_2d::snake_controller::snake_controller_observers::OnSnakeMove;

pub struct GameController2D<const W: usize, const H: usize> {
    game_state: Weak<RefCell<GameState<W, H>>>,
    observers: Vec<Box<dyn OnSnakeMove>>,
}

impl<const W: usize, const H: usize> GameController2D<W, H> {
    pub fn new(game_state: Weak<RefCell<GameState<W, H>>>) -> Self {
        Self {
            game_state,
            observers: Vec::new(),
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

    pub fn add_event_handler (&mut self, observer: Box<dyn OnSnakeMove>) {
        self.observers.push(observer);
    }

    pub fn run_event_handlers (&mut self, last_position: Position2D, new_position: Position2D, direction: Direction2D, grow: bool) {
        for observer in &mut self.observers {
            observer.on_event(&last_position, &new_position, &direction, grow);
        }
    }
}
