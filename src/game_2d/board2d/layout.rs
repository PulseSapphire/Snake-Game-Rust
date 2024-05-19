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

    pub fn get_width(&self) -> u8 {
        self.layout.len() as u8
    }
    
    pub fn get_height(&self) -> u8 {
        self.layout[0].len() as u8
    }
}