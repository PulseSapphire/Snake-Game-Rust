use crate::engine_2d::game_controller::GameController2D;
use crate::game::engine::game_state::board::board_tile::BoardTile;
use crate::engine_2d::game_state::board2d::Board2D;
use crate::engine_2d::game_state::food::Food;
use crate::engine_2d::game_state::GameState;
use crate::game::types::Position2D;
use game_state::snake::Snake2D;
use std::cell::RefCell;
use std::rc::Rc;

use crate::game::engine::game_controller::GameController;
use rand::rngs::SmallRng;
use rand::SeedableRng;

pub mod game_controller;
pub mod game_state;

pub struct Engine2D<const W: usize, const H: usize, C: GameController> {
    pub game_state: Rc<RefCell<GameState<W, H>>>,
    pub game_controller: C,
}

impl<const W: usize, const H: usize> Engine2D<W, H, GameController2D<W, H, SmallRng>> {
    const U8_MAX_IN_USIZE: usize = u8::MAX as usize;
    pub fn new(start_position: Position2D, starting_food_position: Position2D) -> Self {
        if W > Self::U8_MAX_IN_USIZE || H > Self::U8_MAX_IN_USIZE {
            panic!("Each dimension can have a max length of 255 only.");
        }

        let snake = Snake2D::new(start_position.clone());
        let food = Food::new(starting_food_position.clone());
        let mut board = Board2D::<W, H>::new();

        board.set_tile_at_pos(&start_position, BoardTile::SnakeTile(0));
        board.set_tile_at_pos(&starting_food_position, BoardTile::FoodTile);

        let state = GameState::<W, H>::new(snake, board, food);
        let game_state = Rc::new(RefCell::new(state));
        let game_controller =
            GameController2D::new(Rc::downgrade(&game_state).clone(), SmallRng::from_entropy());

        Self {
            game_state,
            game_controller,
        }
    }
}
