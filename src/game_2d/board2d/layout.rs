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

}