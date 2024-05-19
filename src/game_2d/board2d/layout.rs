use crate::game::types::Position2D;

pub struct Layout2D<const W: usize, const H: usize>
{
    layout: [[u16; H]; W],
}

impl <const W: usize, const H: usize> Layout2D<W, H>
{
    pub fn new() -> Self {
        Self {
            layout: [[0; H]; W]
        }
    }

    const WIDTH_U8: u8 = W as u8;
    pub fn get_width(&self) -> u8 {
        Self::WIDTH_U8
    }

    const HEIGHT_U8: u8 = H as u8;
    pub fn get_height(&self) -> u8 {
        Self::HEIGHT_U8
    }

    pub fn get_max_adjacent_value (&self, position: &Position2D) -> u16 {
        let x = position.x as usize;
        let y = position.y as usize;

        self.layout[x+1][y]
            .max(self.layout[x-1][y]
                .max(self.layout[x][y+1]
                    .max(self.layout[x][y-1])))

    }

    pub fn get_min_adjacent_value (&self, position: &Position2D) -> u16 {
        let x = position.x as usize;
        let y = position.y as usize;

        self.layout[x+1][y]
            .min(self.layout[x-1][y]
                .min(self.layout[x][y+1]
                    .min(self.layout[x][y-1])))

    }

}