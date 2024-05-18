use crate::cli_game::snake::Snake;
use crate::cli_game::types::Position;

const BOARD_WIDTH: u8 = u8::MAX;
const BOARD_HEIGHT: u8 = u8::MAX;

pub struct Board {
    snake: Snake,
    layout: [[u16; BOARD_HEIGHT as usize]; BOARD_WIDTH as usize],
    food: Position,
}