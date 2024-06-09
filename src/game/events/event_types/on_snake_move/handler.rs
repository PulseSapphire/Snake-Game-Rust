use crate::game::events::event_types::on_snake_move::event::SnakeMoveEvent;
use crate::game::events::EventHandler;
use crate::game::types::position::Position;

pub trait OnSnakeMoveHandler<'a, P: Position + 'a>:
    EventHandler<SnakeMoveEvent<'a, P>>
{
    fn on_snake_move_event(&self, event: &mut SnakeMoveEvent<'a, P>);
}

impl<'a, T, P> EventHandler<SnakeMoveEvent<'a, P>> for T
where
    T: OnSnakeMoveHandler<'a, P>,
    P: Position,
{
    fn on_event(&self, event: &mut SnakeMoveEvent<'a, P>) {
        self.on_snake_move_event(event);
    }
}
