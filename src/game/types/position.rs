use std::hash::Hash;

pub trait Position: Clone + PartialEq + Eq + Hash {}

#[derive(Clone, PartialEq, Debug, Eq, Hash)]
pub struct Position2D {
    pub x: u8,
    pub y: u8,
}

impl Position for Position2D {}