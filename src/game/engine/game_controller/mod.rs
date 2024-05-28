use crate::game::engine::game_controller::food_controller::FoodController;
use crate::game::engine::game_controller::movement_controller::MovementController;

pub mod food_controller;
pub mod movement_controller;

pub trait GameController: MovementController + FoodController {}
