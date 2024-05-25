pub mod game_controller_observers;

use std::cell::RefCell;
use std::rc::Weak;
use crate::game::types::{Direction2D, Position2D};
use crate::engine_2d::game_state::GameState;

use crate::engine_2d::game_controller::game_controller_observers::OnSnakeMove;
use crate::engine_2d::game_state::board2d::Board2D;
use crate::engine_2d::game_state::board2d::board_tile::BoardTile;
use crate::engine_2d::game_state::board2d::board_tile::BoardTileEnum::SnakeTile;
use crate::engine_2d::game_state::snake::Snake2D;
use crate::game::Game;
use crate::game::types::Direction2D::{Down, Left, Right, Stationary, Up};

pub struct GameController2D<const W: usize, const H: usize> {
    game_state: Weak<RefCell<GameState<W, H>>>,
    observers: Vec<Box<dyn OnSnakeMove<W, H>>>,
}

impl<const W: usize, const H: usize> GameController2D<W, H> {
    pub fn new(game_state: Weak<RefCell<GameState<W, H>>>) -> Self {
        Self {
            game_state,
            observers: Vec::new(),
        }
    }

    fn move_if_valid_direction (dir: Direction2D, width: u8, height: u8, hx: &mut u8, hy: &mut u8) -> Result<(), &'static str> {
        use Direction2D::*;
        match dir {
            Up => {
                if *hy == 0 {
                    return Err("Snake hits the top wall!");
                }

                *hy -= 1;
            }

            Down => {
                if *hy == height - 1 {
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
                if *hx == width - 1 {
                    return Err("Snake hits the right wall!");
                }

                *hx += 1;
            }

            Stationary => (),
        }

        Ok(())
    }

    fn move_head(snake: &mut Snake2D, board: &mut Board2D<W, H>) -> Result<bool, &'static str> {
        let mut grows = false;
        let dir = snake.get_direction().clone();
        let Position2D {
            x: ref mut hx,
            y: ref mut hy,
        } = snake.get_head_position_mut();

        let prev_hx = hx.clone();
        let prev_hy = hy.clone();

        Self::move_if_valid_direction(dir, board.get_width(), board.get_height(), hx, hy)?;

        let hx = *hx as usize;
        let hy = *hy as usize;

        match board.get_tile_at_index(hx, hy) {
            BoardTile::FoodTile => {
                grows = true;
            },
            BoardTile::SnakeTile(_) => {
                return Err("Snake hits itself!");
            },
            BoardTile::EmptyTile => (),
        }

        let prev_head_val = if let BoardTile::SnakeTile(val) = board.get_tile_at_index(prev_hx as usize, prev_hy as usize) {
            val.clone()
        } else {
            panic!("Tile at given index is not a snake tile.");
        };

        board.set_tile_at_index(hx, hy, SnakeTile(prev_head_val + 1));

        Ok(grows)
    }

    fn move_tail(snake: &mut Snake2D, board: &mut Board2D<W, H>) {
        let current_tail_pos = snake.get_tail_position();
        let current_val = if let SnakeTile(value) = board.get_tile_at_pos(current_tail_pos) {
            value
        } else {
            panic!("Snake's tail position is not valid for board.")
        };

        board.set_tile_at_pos(current_tail_pos, SnakeTile(0));

        if let Some(position) = board.get_adjacent_snake_tile_with_value(current_tail_pos, current_val + 1)
        {
            snake.set_tail_position(position);
        } else {
            panic!("Could not find new valid tail position for snake.");
        }
    }

    pub fn move_snake(&mut self) -> Result<(), &'static str> {
        let mut state = if let Some(state) = self.game_state.upgrade() {
            state.borrow_mut()
        } else {
            panic!("Failed to get a reference to the game state.")
        };

        let snake = state.get_mut_snake();
        if let Stationary = snake.get_direction() {
            return Ok(());
        }

        let board = state.get_mut_board();

        let last_head_pos = snake.get_head_position().clone();
        let last_tail_pos = snake.get_tail_position().clone();

        let can_move_tail = Self::move_head(snake, board)?;
        if can_move_tail {
            self.move_tail();
            snake.increment_length();
        }

        self.run_event_handlers(last_head_pos, snake.get_head_position(), last_tail_pos, snake.get_head_position(), board);

        Ok(())
    }

    pub fn add_event_handler (&mut self, observer: Box<dyn OnSnakeMove<W, H>>) {
        self.observers.push(observer);
    }

    pub fn run_event_handlers (&self, last_head_position: &Position2D, new_head_position: &Position2D, last_tail_position: &Position2D, current_tail_position: &Position2D, board: &Board2D<W, H>) {
        for observer in self.observers {
            observer.on_event(last_head_position, new_head_position, last_tail_position, current_tail_position, board);
        }
    }
}
