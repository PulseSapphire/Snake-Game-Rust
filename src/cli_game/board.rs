use crate::cli_game::snake::Snake;

const BOARD_WIDTH: u8 = u8::MAX;
const BOARD_HEIGHT: u8 = u8::MAX;

pub struct Board {
    snake: Snake,
    layout: [[u16; BOARD_HEIGHT as usize]; BOARD_WIDTH as usize]
}