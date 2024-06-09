use crate::game::engine::game_state::board::board_tile::BoardTile;
use crate::game::engine::game_state::board::iterators::BoardIterator;
use crate::game::engine::game_state::board::Board2D;
use crate::game::types::position::Position2D;

pub struct Board2DIterator<'a, B: Board2D> {
    board: &'a B,
    row: usize,
    col: usize,
}

impl<'a, B: Board2D> Board2DIterator<'a, B> {
    pub fn new(board: &'a B) -> Self {
        Self {
            board,
            row: 0,
            col: 0,
        }
    }
}

impl<'a, B: Board2D> BoardIterator<'a, Position2D> for Board2DIterator<'a, B> {}

impl<'a, B: Board2D> Iterator for Board2DIterator<'a, B> {
    type Item = (Position2D, &'a BoardTile);

    fn next(&mut self) -> Option<Self::Item> {
        if self.col == self.board.get_width() as usize {
            return None;
        }

        let tile = self.board.get_tile_at_index(self.row, self.col);
        let item = (
            Position2D {
                x: self.row as u8,
                y: self.col as u8,
            },
            tile,
        );

        if self.row + 1 == self.board.get_height() as usize {
            self.col += 1;
            self.row = 0;
        } else {
            self.row += 1;
        }

        Some(item)
    }
}
