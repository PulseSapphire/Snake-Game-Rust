use crate::game::engine::game_controller::food_controller::FoodController;
use crate::game::engine::game_controller::movement_controller::MovementController;
use crate::game::Game;

mod movement_controller;
mod food_controller;

pub trait GameController: MovementController + FoodController {}