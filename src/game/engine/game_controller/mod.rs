use crate::game::engine::game_controller::food_controller::FoodController;
use crate::game::engine::game_controller::movement_controller::MovementController;
use crate::game::types::position::Position;

pub mod food_controller;
pub mod movement_controller;

pub trait GameController <P: Position>: MovementController + FoodController<P> {}
