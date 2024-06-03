use crate::game::engine::game_state::board::board_tile::BoardTile;
use crate::game::types::position::Position;

pub trait BoardIterator <'a, P: Position> : Iterator<Item = (P, &'a BoardTile)> {}
pub trait BoardIteratorMut <'a, P: Position> : Iterator<Item = (P, &'a mut BoardTile)> {}