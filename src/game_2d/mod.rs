use std::cell::RefCell;
use std::rc::Rc;
use crate::game::types::Position2D;
use game_state::snake::Snake2D;
use crate::game_2d::game_state::board2d::Board2D;
use crate::game_2d::game_state::food::Food;
use crate::game_2d::game_state::GameState;
use crate::game_2d::game_controller::GameController2D;

pub mod game_controller;
mod game_state;

pub struct Game2D<const W: usize, const H: usize> {
    game_state: Rc<RefCell<GameState<W, H>>>,
    game_controller: GameController2D<W, H>,
}

impl <const W: usize, const H: usize> Game2D<W, H> {

    const U8_MAX_IN_USIZE: usize = u8::MAX as usize;
    pub fn new (start_position: Position2D, starting_food_position: Position2D) -> Self {
        if W > Self::U8_MAX_IN_USIZE || H > Self::U8_MAX_IN_USIZE {
            panic!("Each dimension of the game can have a max height and width of 255 only.");
        }

        let snake = Snake2D::new(start_position);
        let food = Food::new(starting_food_position);
        let board = Board2D::<W, H>::new();

        let state = GameState::<W, H>::new(snake, board, food);
        let game_state = Rc::new(RefCell::new(state));

        Self {
            game_state,
            game_controller: GameController2D::new(Rc::downgrade(&game_state))
        }
    }
}