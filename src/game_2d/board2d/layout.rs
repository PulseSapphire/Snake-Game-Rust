pub struct Layout2D<const W: u8, const H: u8> {
    layout: [[u16; H as usize]; W as usize],
}