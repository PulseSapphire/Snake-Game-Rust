use food::Food;
use snake::Snake2D;
use crate::game::engine::game_state::board::Board2D;

pub mod board_2d;
pub mod food;
pub mod snake;

pub struct GameState2D<B: Board2D> {
    snake: Snake2D,
    board: B,
    food: Food,
}

impl<B: Board2D> GameState2D<B> {
    pub fn new(snake: Snake2D, board: B, food: Food) -> Self {
        Self { snake, board, food }
    }

    pub fn get_snake(&self) -> &Snake2D {
        &self.snake
    }
    pub fn get_board(&self) -> &B {
        &self.board
    }
    pub fn get_food(&self) -> &Food {
        &self.food
    }

    pub fn get_mut_snake(&mut self) -> &mut Snake2D {
        &mut self.snake
    }
    pub fn get_mut_board(&mut self) -> &mut B {
        &mut self.board
    }
    pub fn get_mut_food(&mut self) -> &mut Food {
        &mut self.food
    }

    pub fn get_mut_all_fields(&mut self) -> (&mut Snake2D, &mut B, &mut Food) {
        (&mut self.snake, &mut self.board, &mut self.food)
    }
}
