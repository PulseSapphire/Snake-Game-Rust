use crate::engine_2d::game_state::board_2d::Board2D;

pub struct Board2DIterator<'a, const W: usize, const H: usize> {
    board: &'a Board2D<W, H>,
    row: usize,
    col: usize
}

impl <'a, const W: usize, const H: usize> Board2DIterator<'a, W, H> {
    pub fn new (board: &'a Board2D<W, H>) -> Self {
        Self {
            board,
            row: 0,
            col: 0,
        }
    }
}