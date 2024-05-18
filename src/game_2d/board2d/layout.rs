use std::ptr::write_unaligned;

pub struct Layout2D<const W: u8, const H: u8> {
    layout: [[u16; H as usize]; W as usize],
}

impl <const W: u8, const H: u8> Layout2D<W, H> {
    pub fn new() -> Self {
        Self {
            layout: [[0; H as usize]; W as usize]
        }
    }

}