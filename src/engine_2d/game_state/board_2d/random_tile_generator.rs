use crate::game::engine::game_state::board::board_tile::BoardTile;
use crate::game::types::position::Position;
use rand::Rng;
use std::cell::RefCell;
use std::collections::HashMap;

pub struct RandomTileGenerator<R: Rng, P: Position> {
    free_cells: RefCell<(Vec<P>, HashMap<P, usize>)>,

    rng: R,
    tile_type: BoardTile,
}
