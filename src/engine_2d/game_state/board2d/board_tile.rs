#[derive(Clone, Copy)]
pub enum BoardTile {
    FoodTile,
    SnakeTile(u16),
    EmptyTile,
}
