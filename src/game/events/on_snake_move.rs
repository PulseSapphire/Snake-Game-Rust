use crate::game::engine::game_state::board::Board;
use crate::game::types::position::Position;


pub trait OnSnakeMove<P: Position, B: Board<P>> {
    fn on_event(
        &self,
        last_head_position: &P,
        new_head_position: &P,
        last_tail_position: &P,
        current_tail_position: &P,
        board: &B,
    );
}