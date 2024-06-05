pub mod on_snake_move;
pub mod on_board_tile_change;
pub mod event_types;


pub trait Event {}
pub trait CancellableEvent: Event {
    fn is_cancelled (&self) -> bool;
    fn cancel_event(&self);
}

pub trait EventHandler <E: Event> {

    fn on_event(&self, event: &mut E);

}

pub enum EventError {
    AlreadyRegistered,
    DoesNotExist,
}

pub trait EventSubject<E: Event, H: EventHandler<E> + ?Sized> {

    fn add_event_handler (&mut self, event_handler: &H) -> Result<(), EventError>;

    fn remove_event_handler (&mut self, event_handler: &H) -> Result<(), EventError>;

}