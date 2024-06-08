use crate::game::events::event_types::on_board_tile_change::event::BoardTileChangeEvent;
use crate::game::events::EventHandler;
use crate::game::types::position::Position;

pub trait OnBoardTileChangeHandler<'a, P: Position + 'a>:
    EventHandler<BoardTileChangeEvent<'a, P>>
{
    fn on_event(&self, event: &mut BoardTileChangeEvent<'a, P>);
}

impl<'a, T, P> EventHandler<BoardTileChangeEvent<'a, P>> for T
where
    T: OnBoardTileChangeHandler<'a, P>,
    P: Position,
{
    fn on_event(&self, event: &mut BoardTileChangeEvent<'a, P>) {
        OnBoardTileChangeHandler::on_event(self, event);
    }
}
