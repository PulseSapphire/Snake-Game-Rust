#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BoardTile {
    FoodTile,
    SnakeTile(u16),
    EmptyTile,
}
