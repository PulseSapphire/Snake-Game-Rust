#[derive(Clone, Copy, PartialEq)]
pub enum BoardTile {
    FoodTile,
    SnakeTile(u16),
    EmptyTile,
}
