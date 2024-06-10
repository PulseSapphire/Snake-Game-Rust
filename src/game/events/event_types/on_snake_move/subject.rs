use crate::game::events::event_types::on_snake_move::event::SnakeMoveEvent;
use crate::game::events::event_types::on_snake_move::handler::OnSnakeMoveHandler;
use crate::game::events::{EventError, EventSubject};
use crate::game::types::position::Position;

pub trait OnSnakeMoveSubject<P: Position>:
    for<'b> EventSubject<SnakeMoveEvent<'b, P>, dyn OnSnakeMoveHandler<P>>
{
    fn add_on_snake_move_event_handler(
        &mut self,
        event_handler: &dyn OnSnakeMoveHandler<P>,
    ) -> Result<(), EventError>;

    fn remove_on_snake_move_event_handler(
        &mut self,
        event_handler: &dyn OnSnakeMoveHandler<P>,
    ) -> Result<(), EventError>;
}

impl<T, P> EventSubject<SnakeMoveEvent<'_, P>, dyn OnSnakeMoveHandler<P>> for T
where
    T: OnSnakeMoveSubject<P>,
    P: Position,
{
    fn add_event_handler(
        &mut self,
        event_handler: &dyn OnSnakeMoveHandler<P>,
    ) -> Result<(), EventError> {
        self.add_on_snake_move_event_handler(event_handler)
    }

    fn remove_event_handler(
        &mut self,
        event_handler: &dyn OnSnakeMoveHandler<P>,
    ) -> Result<(), EventError> {
        self.remove_on_snake_move_event_handler(event_handler)
    }
}
