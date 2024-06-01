use crate::engine_2d::game_state::board_2d::Board2D;
use crate::game::types::position::Position;

pub trait OnSnakeMove<const W: usize, const H: usize, P: Position> {
    fn on_event(
        &self,
        last_head_position: &P,
        new_head_position: &P,
        last_tail_position: &P,
        current_tail_position: &P,
        board: &Board2D<W, H>,
    );
}