pub mod engine;
mod renderer;

pub mod types;
pub mod events;

pub trait Game {
    fn start();
}
