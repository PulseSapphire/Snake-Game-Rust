use crate::engine_2d::game_state::GameState2D;
use crate::game::types::direction::Direction2D;
use rand::Rng;
use std::cell::RefCell;
use std::rc::Weak;

use crate::engine_2d::game_state::snake::Snake2D;
use crate::game::engine::game_controller::food_controller::FoodController;
use crate::game::engine::game_controller::movement_controller::MovementController;
use crate::game::engine::game_controller::GameController;
use crate::game::engine::game_state::board::board_tile::BoardTile;
use crate::game::engine::game_state::board::Board2D;
use crate::game::events::event_types::on_snake_move::event::SnakeMoveEvent;
use crate::game::events::event_types::on_snake_move::handler::OnSnakeMoveHandler;
use crate::game::types::direction::Direction2D::Stationary;
use crate::game::types::position::Position2D;

pub struct GameController2D<'a, R: Rng, B: Board2D> {
    game_state: Weak<RefCell<GameState2D<B>>>,
    observers: Vec<&'a dyn OnSnakeMoveHandler<Position2D>>,

    rng: R,
}

// impl block for instantiation methods.
impl<'a, R: Rng, B: Board2D> GameController2D<'a, R, B> {
    pub fn new(game_state: Weak<RefCell<GameState2D<B>>>, rng: R) -> Self {
        Self {
            game_state,
            observers: Vec::new(),
            rng,
        }
    }
}

// impl block for movement methods.
impl<'a, R: Rng, B: Board2D> GameController2D<'a, R, B> {
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

    fn move_head(snake: &mut Snake2D, board: &mut B) -> Result<bool, &'static str> {
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

    fn move_tail(snake: &mut Snake2D, board: &mut B) {
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
}

// impl blocks for GameController trait implementation and its sub traits' implementations.
impl<'a, R: Rng, B: Board2D> GameController<Position2D> for GameController2D<'a, R, B> {}
impl<'a, R: Rng, B: Board2D> MovementController for GameController2D<'a, R, B> {
    fn move_snake(&mut self) -> Result<(), &'static str> {
        let state_ref = self
            .game_state
            .upgrade()
            .expect("Failed to get a reference to the game state.");

        let mut state = state_ref.borrow_mut();

        let (snake, board, _) = state.get_mut_all_fields();
        if let Stationary = snake.get_direction() {
            return Ok(());
        }

        let last_head_pos = snake.get_head_position().clone();
        let last_tail_pos = snake.get_tail_position().clone();

        let grows = Self::move_head(snake, board)?;
        if grows {
            snake.increment_length();
        } else {
            Self::move_tail(snake, board);
        }

        self.run_event_handlers(
            &last_head_pos,
            snake.get_head_position(),
            &last_tail_pos,
            snake.get_head_position(),
            snake.get_length(),
        );

        Ok(())
    }
}
impl<'a, R: Rng, B: Board2D> FoodController<Position2D> for GameController2D<'a, R, B> {
    fn spawn_food(&mut self, position: &Position2D) -> Result<(), &'static str> {
        let state_ref = self
            .game_state
            .upgrade()
            .expect("Failed to get a reference to the game state.");

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
        let state_ref = self
            .game_state
            .upgrade()
            .expect("Failed to get a reference to the game state.");

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

impl<'a, R: Rng, B: Board2D> GameController2D<'a, R, B> {
    pub fn add_event_handler(&mut self, observer: &'a dyn OnSnakeMoveHandler<Position2D>) {
        self.observers.push(observer);
    }

    fn run_event_handlers(
        &self,
        last_head_position: &Position2D,
        new_head_position: &Position2D,
        last_tail_position: &Position2D,
        current_tail_position: &Position2D,
        length: u16,
    ) {
        let mut event = SnakeMoveEvent::new(
            last_head_position,
            new_head_position,
            last_tail_position,
            current_tail_position,
            length
        );
        for observer in &self.observers {
            observer.on_event(&mut event);
        }
    }
}