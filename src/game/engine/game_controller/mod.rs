pub trait GameController {
    fn move_snake(&mut self);
    fn spawn_food(&mut self);
}