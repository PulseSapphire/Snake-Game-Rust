use crate::game::types::Position2D;

pub struct Layout2D<const W: usize, const H: usize> {
    layout: [[u16; H]; W],
}

impl<const W: usize, const H: usize> Layout2D<W, H> {
    pub fn new() -> Self {
        Self {
            layout: [[0; H]; W],
        }
    }

    pub fn get_val_at_pos(&self, position: &Position2D) -> u16 {
        self.layout[position.x as usize][position.y as usize]
    }

    pub fn get_val_at_index(&self, x: usize, y: usize) -> u16 {
        self.layout[x][y]
    }

    pub fn set_val_at_index(&mut self, x: usize, y: usize, new_value: u16) {
        self.layout[x][y] = new_value;
    }

    pub fn get_layout(&self) -> &[[u16; H]] {
        &self.layout
    }

    const WIDTH_U8: u8 = W as u8;
    pub fn get_width(&self) -> u8 {
        Self::WIDTH_U8
    }

    const HEIGHT_U8: u8 = H as u8;
    pub fn get_height(&self) -> u8 {
        Self::HEIGHT_U8
    }

    pub fn get_max_adjacent_value(&self, position: &Position2D) -> u16 {
        let x = position.x as usize;
        let y = position.y as usize;

        self.layout[x + 1][y]
            .max(self.layout[x - 1][y].max(self.layout[x][y + 1].max(self.layout[x][y - 1])))
    }

    pub fn get_min_adjacent_value(&self, position: &Position2D) -> u16 {
        let x = position.x as usize;
        let y = position.y as usize;

        self.layout[x + 1][y]
            .min(self.layout[x - 1][y].min(self.layout[x][y + 1].min(self.layout[x][y - 1])))
    }

    pub fn get_max_adjacent_position(&self, position: &Position2D) -> Position2D {
        let x = position.x as usize;
        let y = position.y as usize;

        let mut max_pos = Position2D { x: 0, y: 0 };
        let mut max_val = u16::MIN;

        let adjacent_positions = [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)];

        for (adj_x, adj_y) in adjacent_positions {
            let val = self.layout[adj_x][adj_y];
            if val > max_val {
                max_val = val;
                max_pos = Position2D {
                    x: adj_x as u8,
                    y: adj_y as u8,
                };
            }
        }

        max_pos
    }

    pub fn get_min_adjacent_position(&self, position: &Position2D) -> Position2D {
        let x = position.x as usize;
        let y = position.y as usize;

        let mut min_pos = Position2D { x: 0, y: 0 };
        let mut min_val = u16::MIN;

        let adjacent_positions = [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)];

        for (adj_x, adj_y) in adjacent_positions {
            let val = self.layout[adj_x][adj_y];
            if val < min_val {
                min_val = val;
                min_pos = Position2D {
                    x: adj_x as u8,
                    y: adj_y as u8,
                };
            }
        }

        min_pos
    }

    pub fn get_adjacent_position_with_val(
        &self,
        position: &Position2D,
        target_value: u16,
    ) -> Option<Position2D> {
        let x = position.x as usize;
        let y = position.y as usize;

        let adjacent_positions = [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)];

        for (adj_x, adj_y) in adjacent_positions {
            let val = self.layout[adj_x][adj_y];
            if val == target_value {
                return Some(Position2D {
                    x: adj_x as u8,
                    y: adj_y as u8,
                });
            }
        }

        None
    }
}
