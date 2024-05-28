pub mod engine;
mod renderer;

pub mod types;

pub trait Game {
    fn start();
}
