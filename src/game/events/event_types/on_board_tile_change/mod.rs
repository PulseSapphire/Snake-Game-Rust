pub mod handler;
pub mod subject;
pub mod event;

use event::BoardTileChangeEvent;
use crate::game::events::{Event, EventError, EventHandler, EventSubject};
use crate::game::types::position::Position;

pub trait OnBoardTileChangeHandler<'a, P: Position + 'a>: EventHandler<BoardTileChangeEvent<'a, P>> {
    fn on_event(&self, event: &mut BoardTileChangeEvent<'a, P>);
}
impl <'a, T, P> EventHandler<BoardTileChangeEvent<'a, P>> for T
where
    T: OnBoardTileChangeHandler<'a, P>,
    P: Position
{
    fn on_event(&self, event: &mut BoardTileChangeEvent<'a, P>) {
        OnBoardTileChangeHandler::on_event(self, event);
    }
}

pub trait OnBoardTileChangeSubject <'a, P: Position + 'a>: EventSubject<BoardTileChangeEvent<'a, P>, dyn OnBoardTileChangeHandler<'a, P>> {
    fn add_event_handler (&mut self, event_handler: &dyn OnBoardTileChangeHandler<P>) -> Result<(), EventError>;

    fn remove_event_handler (&mut self, event_handler: &dyn OnBoardTileChangeHandler<P>) -> Result<(), EventError>;
}
impl <'a, T, P>
EventSubject<BoardTileChangeEvent<'a, P>, dyn OnBoardTileChangeHandler<'a, P>>
for T
where
    T: OnBoardTileChangeSubject<'a, P>,
    P: Position
{

    fn add_event_handler (&mut self, event_handler: &dyn OnBoardTileChangeHandler<P>) -> Result<(), EventError> {
        OnBoardTileChangeSubject::add_event_handler(self, event_handler)
    }

    fn remove_event_handler (&mut self, event_handler: &dyn OnBoardTileChangeHandler<P>) -> Result<(), EventError> {
        OnBoardTileChangeSubject::remove_event_handler(self, event_handler)
    }

}
