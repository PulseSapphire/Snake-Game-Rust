pub mod event_manager;
pub mod event_types;

pub trait Event {}
pub trait CancellableEvent: Event {
    fn is_cancelled(&self) -> bool;
    fn cancel_event(&self);
}

pub trait EventHandler<E: Event> {
    fn on_event(&self, event: &mut E);
}

pub enum EventError {
    AlreadyRegistered,
    DoesNotExist,
}

pub trait EventSubject<'a, E: Event, H: EventHandler<E> + ?Sized> {
    fn add_event_handler(&mut self, event_handler: &'a H) -> Result<(), EventError>;

    fn remove_event_handler(&mut self, event_handler: &'a H) -> Result<(), EventError>;
}
