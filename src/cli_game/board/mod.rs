use layout::Layout;
use crate::cli_game::snake::Snake;
use crate::cli_game::types::Position;

mod layout;

const BOARD_WIDTH: u8 = u8::MAX;
const BOARD_HEIGHT: u8 = u8::MAX;

pub struct Board {
    snake: Snake,
    layout: Layout,
    food: Position,
}
