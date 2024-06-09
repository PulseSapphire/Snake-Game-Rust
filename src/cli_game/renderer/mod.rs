use crate::game::engine::game_state::board::Board2D;
use crate::game::events::event_types::on_board_tile_change::event::BoardTileChangeEvent;
use crate::game::events::event_types::on_snake_move::handler::OnSnakeMoveHandler;
use crate::game::types::position::Position2D;

pub struct CLIRenderer;

impl CLIRenderer {

    pub fn new () -> Self {
        Self {}
    }

}