pub type BoardTile = BoardTileEnum;

pub enum BoardTileEnum {
    FoodTile,
    SnakeTile(u16),
    EmptyTile,
}