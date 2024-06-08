use crate::game::events::{Event, EventHandler};
use std::collections::HashSet;

pub struct EventManager<'a, E: Event> {
    observers: HashSet<&'a dyn EventHandler<E>>,
}

impl<'a, E: Event> EventManager<'a, E> {
    pub fn new() -> Self {
        Self {
            observers: HashSet::new(),
        }
    }
}
