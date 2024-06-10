use crate::game::events::event_types::on_snake_move::event::SnakeMoveEvent;
use crate::game::events::EventHandler;
use crate::game::types::position::Position;

pub trait OnSnakeMoveHandler<P: Position>:
    for <'a> EventHandler<SnakeMoveEvent<'a, P>>
{
    fn on_snake_move_event(&self, event: &mut SnakeMoveEvent<P>);
}

impl<T, P> EventHandler<SnakeMoveEvent<'_, P>> for T
where
    T: OnSnakeMoveHandler<P>,
    P: Position,
{
    fn on_event(&self, event: &mut SnakeMoveEvent<P>) {
        self.on_snake_move_event(event);
    }
}
