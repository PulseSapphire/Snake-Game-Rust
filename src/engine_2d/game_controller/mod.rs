use crate::engine_2d::game_state::GameState;
use crate::game::types::direction::Direction2D;
use rand::Rng;
use std::cell::RefCell;
use std::rc::Weak;

use crate::game::events::on_snake_move::OnSnakeMove;
use crate::game::engine::game_state::board::board_tile::BoardTile;
use crate::engine_2d::game_state::board_2d::Board2D;
use crate::engine_2d::game_state::snake::Snake2D;
use crate::game::engine::game_controller::food_controller::FoodController;
use crate::game::engine::game_controller::movement_controller::MovementController;
use crate::game::engine::game_controller::GameController;
use crate::game::engine::game_state::board::Board;
use crate::game::types::direction::Direction2D::Stationary;
use crate::game::types::position::Position2D;

pub struct GameController2D<const W: usize, const H: usize, R: Rng> {
    game_state: Weak<RefCell<GameState<W, H>>>,
    observers: Vec<Box<dyn OnSnakeMove<Position2D, Board2D<W, H>>>>,

    rng: R,
}

impl<const W: usize, const H: usize, R: Rng> GameController2D<W, H, R> {
    pub fn new(game_state: Weak<RefCell<GameState<W, H>>>, rng: R) -> Self {
        Self {
            game_state,
            observers: Vec::new(),
            rng,
        }
    }

    fn move_if_valid_direction(
        dir: Direction2D,
        width: u8,
        height: u8,
        hx: &mut u8,
        hy: &mut u8,
    ) -> Result<(), &'static str> {
        use crate::game::types::direction::Direction2D::*;
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

        let prev_hx = hx.clone() as usize;
        let prev_hy = hy.clone() as usize;

        Self::move_if_valid_direction(dir, board.get_width(), board.get_height(), hx, hy)?;

        let hx = *hx as usize;
        let hy = *hy as usize;

        match board.get_tile_at_index(hx, hy) {
            BoardTile::FoodTile => {
                grows = true;
            }
            BoardTile::SnakeTile(_) => {
                return Err("Snake hits itself!");
            }
            BoardTile::EmptyTile => (),
        }

        let prev_head_val =
            if let BoardTile::SnakeTile(val) = board.get_tile_at_index(prev_hx, prev_hy) {
                val.clone()
            } else {
                panic!("Tile at given index is not a snake tile.");
            };

        board.set_tile_at_index(hx, hy, BoardTile::SnakeTile(prev_head_val + 1));

        Ok(grows)
    }

    fn move_tail(snake: &mut Snake2D, board: &mut Board2D<W, H>) {
        let current_tail_pos = snake.get_tail_position();
        let current_val =
            if let BoardTile::SnakeTile(value) = board.get_tile_at_pos(current_tail_pos) {
                value.clone()
            } else {
                panic!("Snake's tail position is not valid for board.")
            };

        board.set_tile_at_pos(current_tail_pos, BoardTile::EmptyTile);

        if let Some(position) =
            board.get_adjacent_snake_tile_with_value(current_tail_pos, current_val + 1)
        {
            snake.set_tail_position(position);
        } else {
            panic!("Could not find new valid tail position for snake.");
        }
    }

    pub fn add_event_handler(&mut self, observer: Box<dyn OnSnakeMove<Position2D, Board2D<W, H>>>) {
        self.observers.push(observer);
    }

    pub fn run_event_handlers(
        &self,
        last_head_position: &Position2D,
        new_head_position: &Position2D,
        last_tail_position: &Position2D,
        current_tail_position: &Position2D,
        board: &Board2D<W, H>,
    ) {
        for observer in &self.observers {
            observer.on_event(
                last_head_position,
                new_head_position,
                last_tail_position,
                current_tail_position,
                board,
            );
        }
    }
}

impl<const W: usize, const H: usize, R: Rng> GameController<Position2D> for GameController2D<W, H, R> {}
impl<const W: usize, const H: usize, R: Rng> MovementController for GameController2D<W, H, R> {
    fn move_snake(&mut self) -> Result<(), &'static str> {
        let state_ref = self.game_state.upgrade().expect("Failed to get a reference to the game state.");

        let mut state = state_ref.borrow_mut();

        let (snake, board, _) = state.get_mut_all_fields();
        if let Stationary = snake.get_direction() {
            return Ok(());
        }

        let last_head_pos = snake.get_head_position().clone();
        let last_tail_pos = snake.get_tail_position().clone();

        let can_move_tail = !Self::move_head(snake, board)?;
        if can_move_tail {
            Self::move_tail(snake, board);
            snake.increment_length();
        }

        self.run_event_handlers(
            &last_head_pos,
            snake.get_head_position(),
            &last_tail_pos,
            snake.get_head_position(),
            board,
        );

        Ok(())
    }
}
impl<const W: usize, const H: usize, R: Rng> FoodController<Position2D> for GameController2D<W, H, R> {
    fn spawn_food(&mut self, position: &Position2D) -> Result<(), &'static str> {
        let state_ref = self.game_state.upgrade().expect("Failed to get a reference to the game state.");

        let mut state = state_ref.borrow_mut();

        let (_, board, food) = state.get_mut_all_fields();

        if board.get_tile_at_pos(position) != &BoardTile::EmptyTile {
            return Err("Cannot spawn food at given position because it is occupied.");
        }

        food.set_position(position.clone());
        board.set_tile_at_pos(&position, BoardTile::FoodTile);

        Ok(())
    }

    fn spawn_food_random(&mut self) {
        let state_ref = self.game_state.upgrade().expect("Failed to get a reference to the game state.");

        let mut state = state_ref.borrow_mut();

        let (_, board, food) = state.get_mut_all_fields();

        let mut position = Position2D {
            x: self.rng.gen_range(0..board.get_width()),
            y: self.rng.gen_range(0..board.get_height()),
        };
        while board.get_tile_at_pos(&position) != &BoardTile::EmptyTile {
            position = Position2D {
                x: self.rng.gen_range(0..board.get_width()),
                y: self.rng.gen_range(0..board.get_height()),
            };
        }

        food.set_position(position.clone());
        board.set_tile_at_pos(&position, BoardTile::FoodTile);
    }
}
