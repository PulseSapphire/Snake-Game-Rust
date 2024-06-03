use crate::engine_2d::game_state::board_2d::Board2D;
use crate::game::engine::game_state::board::board_tile::BoardTile;
use crate::game::engine::game_state::board::iterators::BoardIterator;
use crate::game::types::position::Position2D;

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

impl <'a, const W: usize, const H: usize> BoardIterator<'a, Position2D> for Board2DIterator<'a, W, H> {}

impl <'a, const W: usize, const H: usize> Iterator for Board2DIterator<'a, W, H> {

    type Item = (Position2D, &'a BoardTile);

    fn next(&mut self) -> Option<Self::Item> {
        if self.col > W {
            return None;
        }

        let tile = self.board.get_tile_at_index(self.row, self.col);
        let item = (Position2D { x: self.row as u8, y: self.col as u8 }, tile);

        if self.row + 1 == H {
            self.col += 1;
            self.row = 0;
        }
        else {
            self.row += 1;
        }

        Some(item)
    }

}