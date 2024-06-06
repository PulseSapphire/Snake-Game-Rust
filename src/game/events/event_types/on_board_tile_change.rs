use crate::game::engine::game_state::board::board_tile::BoardTile;
use crate::game::events::{Event, EventHandler};
use crate::game::types::position::Position;

pub struct BoardTileChangeEvent<'a, P: Position + 'a> {
    new_board_tile: &'a BoardTile,
    last_board_tile: &'a BoardTile,
    position: &'a P,
}

impl<'a, P: Position + 'a> BoardTileChangeEvent<'a, P> {
    pub fn get_new_board_tile(&self) -> &'a BoardTile {
        self.new_board_tile
    }
    pub fn get_last_board_tile(&self) -> &'a BoardTile {
        self.last_board_tile
    }
    pub fn get_position(&self) -> &'a P {
        self.position
    }
}

impl<'a, P: Position> Event for BoardTileChangeEvent<'a, P> {}
pub trait OnBoardTileChange <'a, P: Position + 'a>: EventHandler<BoardTileChangeEvent<'a, P>> {}