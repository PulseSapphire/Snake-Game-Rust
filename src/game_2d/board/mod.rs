use layout::Layout;
use crate::cli_game::snake::Snake;
use crate::game::types::Position2D;

mod layout;

const BOARD_WIDTH: u8 = u8::MAX;
const BOARD_HEIGHT: u8 = u8::MAX;

pub struct Board2D {
    snake: Snake,
    layout: Layout,
    food: Position2D,
}
