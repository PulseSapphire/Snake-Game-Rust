use crate::game::engine::game_state::board::board_tile::BoardTile;
use crate::game::types::position::Position;

pub mod board_tile;

pub trait Board <P: Position> {
    fn get_tile_at_pos(&self, position: &P) -> &BoardTile;

    fn set_tile_at_pos(&mut self, position: &P, new_board_tile: BoardTile);

    fn get_tile_at_index(&self, x: usize, y: usize) -> &BoardTile;

    fn set_tile_at_index(&mut self, x: usize, y: usize, new_tile_value: BoardTile);

    fn get_adjacent_snake_tile_with_value(&self, position: &P, target_value: u16) -> Option<P>;
}