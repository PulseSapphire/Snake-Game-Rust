use crate::cli_game::board::{BOARD_HEIGHT, BOARD_WIDTH};

pub struct Layout {
    layout: [[u16; BOARD_HEIGHT as usize]; BOARD_WIDTH as usize]
}
