pub mod on_snake_move;
pub mod on_board_tile_change;


pub trait Event {}
pub trait CancellableEvent: Event {
    fn is_cancelled () -> bool;
    fn cancel_event();
}

pub trait EventHandler <E: Event> {

    fn on_event(&self, event: E);

}

pub enum EventError {
    AlreadyRegistered,
    DoesNotExist,
}

pub trait EventSubject<E: Event, H: EventHandler<E>> {

    fn add_event_handler (&mut self, event_hanlder: &H) -> Result<(), EventError>;

    fn remove_event_handler (&mut self, event_handler: &H) -> Result<(), EventError>;

}