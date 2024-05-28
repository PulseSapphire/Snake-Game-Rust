use crate::game::types::Position2D;

pub trait FoodController {
    fn spawn_food(&mut self, position: &Position2D);
    fn spawn_food_random(&mut self);
}