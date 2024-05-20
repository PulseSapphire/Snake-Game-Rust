use crate::game::types::Position2D;
use crate::game_2d::game_state::board2d::layout::Layout2D;

mod layout;

pub struct Board2D<const W: usize, const H: usize> {
    layout: Layout2D<W, H>,
    food: Position2D,
}

impl<const W: usize, const H: usize> Board2D<W, H> {
    const U8_MAX_IN_USIZE: usize = u8::MAX as usize;
    pub fn new(food: Position2D) -> Self {
        if W > Self::U8_MAX_IN_USIZE || H > Self::U8_MAX_IN_USIZE {
            panic!(
                "Cannot use dimensions bigger than {}",
                Self::U8_MAX_IN_USIZE
            );
        }

        Self {
            food,
            layout: Layout2D::<W, H>::new(),
        }
    }

    pub fn get_width(&self) -> u8 {
        self.layout.get_width()
    }

    pub fn get_height(&self) -> u8 {
        self.layout.get_height()
    }

    pub fn get_layout(&self) -> &Layout2D<W, H> {
        &self.layout
    }

    pub fn get_layout_mut(&mut self) -> &mut Layout2D<W, H> {
        &mut self.layout
    }
}
