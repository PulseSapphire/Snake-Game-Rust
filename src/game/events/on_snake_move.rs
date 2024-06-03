use crate::game::engine::game_state::board::Board;
use crate::game::events::EventHandler;
use crate::game::types::position::Position;

pub trait OnSnakeMove<const W: usize, const H: usize, P: Position>: EventHandler {
pub trait OnSnakeMove<const W: usize, const H: usize, P: Position, B: Board<P>>: EventHandler {
    fn on_event(
        &self,
        last_head_position: &P,
        new_head_position: &P,
        last_tail_position: &P,
        current_tail_position: &P,
        board: &B,
    );
}