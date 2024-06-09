use crate::game::events::event_types::on_board_tile_change::event::BoardTileChangeEvent;
use crate::game::events::event_types::on_board_tile_change::handler::OnBoardTileChangeHandler;
use crate::game::events::{EventError, EventSubject};
use crate::game::types::position::Position;

pub trait OnBoardTileChangeSubject<'a, P: Position + 'a>:
    EventSubject<BoardTileChangeEvent<'a, P>, dyn OnBoardTileChangeHandler<'a, P>>
{
    fn add_on_board_tile_change_event_handler(
        &mut self,
        event_handler: &dyn OnBoardTileChangeHandler<P>,
    ) -> Result<(), EventError>;

    fn remove_on_board_tile_change_event_handler(
        &mut self,
        event_handler: &dyn OnBoardTileChangeHandler<P>,
    ) -> Result<(), EventError>;
}

impl<'a, T, P> EventSubject<BoardTileChangeEvent<'a, P>, dyn OnBoardTileChangeHandler<'a, P>> for T
where
    T: OnBoardTileChangeSubject<'a, P>,
    P: Position,
{
    fn add_event_handler(
        &mut self,
        event_handler: &dyn OnBoardTileChangeHandler<P>,
    ) -> Result<(), EventError> {
        self.add_on_board_tile_change_event_handler(event_handler)
    }

    fn remove_event_handler(
        &mut self,
        event_handler: &dyn OnBoardTileChangeHandler<P>,
    ) -> Result<(), EventError> {
        self.remove_on_board_tile_change_event_handler(event_handler)
    }
}
