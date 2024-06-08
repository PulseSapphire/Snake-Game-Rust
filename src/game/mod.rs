pub mod engine;
mod renderer;

pub mod events;
pub mod types;

pub trait Game {
    fn start();
}
