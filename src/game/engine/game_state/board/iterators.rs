use crate::game::engine::game_state::board::board_tile::BoardTile;
use crate::game::types::position::Position;

pub trait BoardIterator<'a, P: Position>: Iterator<Item = (P, &'a BoardTile)> {}
pub trait BoardIteratorMut<'a, P: Position>: Iterator<Item = (P, &'a mut BoardTile)> {}

pub trait IterableBoard<'a, P: Position, I: BoardIterator<'a, P>>:
    IntoIterator<Item = (P, &'a BoardTile), IntoIter = I>
{
}
pub trait IterableBoardMut<'a, P: Position, I: BoardIteratorMut<'a, P>>:
    IntoIterator<Item = (P, &'a mut BoardTile), IntoIter = I>
{
}
