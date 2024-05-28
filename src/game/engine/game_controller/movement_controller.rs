pub trait MovementController {
    fn move_snake(&mut self) -> Result<(), &'static str>;
}
