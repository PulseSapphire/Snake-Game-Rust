use crate::engine_2d::game_state::board2d::Board2D;
use crate::game::types::Position2D;

pub trait OnSnakeMove<const W: usize, const H: usize> {
    fn on_event(
        &self,
        last_head_position: &Position2D,
        new_head_position: &Position2D,
        last_tail_position: &Position2D,
        current_tail_position: &Position2D,
        board: &Board2D<W, H>,
    );
}
