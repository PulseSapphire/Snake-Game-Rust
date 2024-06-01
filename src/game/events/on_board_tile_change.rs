use crate::game::engine::game_state::board::board_tile::BoardTile;
use crate::game::types::position::Position;

pub trait OnBoardTileChange <P: Position> {
    fn on_event (new_board_tile: &BoardTile, last_board_tile: &BoardTile, position: &P);
}