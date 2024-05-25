pub type BoardTile = BoardTileEnum;

enum BoardTileEnum {
    FoodTile,
    SnakeTile(u16),
    EmptyTile,
}