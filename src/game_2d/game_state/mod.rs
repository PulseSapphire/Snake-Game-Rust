use crate::game_2d::board2d::Board2D;
use crate::game_2d::food::Food;
use crate::game_2d::snake::Snake2D;

struct GameState <const W: usize, const H: usize> {
    snake: Snake2D,
    board: Board2D<W, H>,
    food: Food
}