use crate::game::events::event_types::on_board_tile_change::event::BoardTileChangeEvent;
use crate::game::events::EventHandler;
use crate::game::types::position::Position;

pub trait OnBoardTileChangeHandler<P: Position>:
    for <'a> EventHandler<BoardTileChangeEvent<'a, P>>
{
    fn on_board_tile_change_event(&self, event: &mut BoardTileChangeEvent<P>);
}

impl<T, P> EventHandler<BoardTileChangeEvent<'_, P>> for T
where
    T: OnBoardTileChangeHandler<P>,
    P: Position,
{
    fn on_event(&self, event: &mut BoardTileChangeEvent<P>) {
        self.on_board_tile_change_event(event);
    }
}
