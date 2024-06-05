use crate::game::engine::game_state::board::board_tile::BoardTile;
use crate::game::events::{Event, EventHandler};
use crate::game::types::position::Position;

pub struct BoardTileChangeEvent<'a, P: Position + 'a> {
    pub new_board_tile: &'a BoardTile,
    pub last_board_tile: &'a BoardTile,
    pub position: &'a P,
}

impl<'a, P: Position> Event for BoardTileChangeEvent<'a, P> {}
pub trait OnBoardTileChange <'a, P: Position + 'a>: EventHandler<BoardTileChangeEvent<'a, P>> {}