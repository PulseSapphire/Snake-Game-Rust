use crate::game::types::position::Position;

pub trait FoodController <P: Position> {
    fn spawn_food(&mut self, position: &P) -> Result<(), &'static str>;
    fn spawn_food_random(&mut self);
}
