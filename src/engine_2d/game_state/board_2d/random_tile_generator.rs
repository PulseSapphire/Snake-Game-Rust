use std::cell::RefCell;
use std::collections::HashMap;
use rand::Rng;
use crate::game::engine::game_state::board::board_tile::BoardTile;
use crate::game::types::position::Position;

pub struct RandomTileGenerator<R: Rng, P: Position> {
    free_cells: RefCell<(Vec<P>, HashMap<P, usize>)>,

    rng: R,
    tile_type: BoardTile
}