pub mod iterators;
pub mod random_tile_generator;

use crate::engine_2d::game_state::board_2d::iterators::Board2DIterator;
use crate::game::engine::game_state::board;
use crate::game::engine::game_state::board::board_tile::BoardTile;
use crate::game::engine::game_state::board::iterators::IterableBoard;
use crate::game::engine::game_state::board::Board;
use crate::game::types::position::Position2D;

pub struct Board2D<const W: usize, const H: usize> {
    layout: [[BoardTile; H]; W],
}

impl<const W: usize, const H: usize> Board<Position2D> for Board2D<W, H> {
    fn get_tile_at_pos(&self, position: &Position2D) -> &BoardTile {
        &self.layout[position.x as usize][position.y as usize]
    }

    fn set_tile_at_pos(&mut self, position: &Position2D, new_board_tile: BoardTile) {
        self.layout[position.x as usize][position.y as usize] = new_board_tile;
    }

    fn get_adjacent_snake_tile_with_value(
        &self,
        position: &Position2D,
        target_value: u16,
    ) -> Option<Position2D> {
        let x = position.x as usize;
        let y = position.y as usize;

        let adjacent_positions = [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)];

        for (adj_x, adj_y) in adjacent_positions {
            if let BoardTile::SnakeTile(value) = self.layout[adj_x][adj_y] {
                if value == target_value {
                    return Some(Position2D {
                        x: adj_x as u8,
                        y: adj_y as u8,
                    });
                }
            }
        }

        None
    }

    fn get_dimensions<'a>() -> &'a [u8] {
        &[Self::WIDTH_U8, Self::HEIGHT_U8]
    }
}

impl<const W: usize, const H: usize> board::Board2D for Board2D<W, H> {
    fn get_width(&self) -> u8 {
        Self::WIDTH_U8
    }
    fn get_height(&self) -> u8 {
        Self::HEIGHT_U8
    }

    fn get_tile_at_index(&self, x: usize, y: usize) -> &BoardTile {
        &self.layout[x][y]
    }

    fn set_tile_at_index(&mut self, x: usize, y: usize, new_tile_value: BoardTile) {
        self.layout[x][y] = new_tile_value;
    }
}

impl<const W: usize, const H: usize> Board2D<W, H> {
    const WIDTH_U8: u8 = W as u8;
    const HEIGHT_U8: u8 = H as u8;

    pub fn new() -> Self {
        Self {
            layout: [[BoardTile::EmptyTile; H]; W],
        }
    }

    pub fn get_layout(&self) -> &[[BoardTile; H]] {
        &self.layout
    }
}

impl<'a, const W: usize, const H: usize> IterableBoard<'a, Position2D, Board2DIterator<'a, Board2D<W, H>>> for &'a Board2D<W, H> {}

impl<'a, const W: usize, const H: usize>
IntoIterator
for &'a Board2D<W, H> {
    type Item = (Position2D, &'a BoardTile);
    type IntoIter = Board2DIterator<'a, Board2D<W, H>>;
    fn into_iter(self) -> Self::IntoIter {
        Board2DIterator::new(self)
    }
}