use crate::engine_2d::game_state::board_2d::Board2D;
use food::Food;
use snake::Snake2D;

pub mod board_2d;
pub mod food;
pub mod snake;

pub struct GameState2D<const W: usize, const H: usize> {
    snake: Snake2D,
    board: Board2D<W, H>,
    food: Food,
}

impl<const W: usize, const H: usize> GameState2D<W, H> {
    pub fn new(snake: Snake2D, board: Board2D<W, H>, food: Food) -> Self {
        Self { snake, board, food }
    }

    pub fn get_snake(&self) -> &Snake2D {
        &self.snake
    }
    pub fn get_board(&self) -> &Board2D<W, H> {
        &self.board
    }
    pub fn get_food(&self) -> &Food {
        &self.food
    }

    pub fn get_mut_snake(&mut self) -> &mut Snake2D {
        &mut self.snake
    }
    pub fn get_mut_board(&mut self) -> &mut Board2D<W, H> {
        &mut self.board
    }
    pub fn get_mut_food(&mut self) -> &mut Food {
        &mut self.food
    }

    pub fn get_mut_all_fields(&mut self) -> (&mut Snake2D, &mut Board2D<W, H>, &mut Food) {
        (&mut self.snake, &mut self.board, &mut self.food)
    }
}
