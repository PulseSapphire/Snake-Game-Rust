use crate::game::types::Position2D;
use crate::game_2d::board2d::layout::Layout2D;
use crate::game_2d::snake::Snake2D;

mod layout;

pub struct Board2D<const W: usize, const H: usize> {
    snake: Snake2D,
    layout: Layout2D<W, H>,
    food: Position2D,
}

impl<const W: usize, const H: usize> Board2D<W, H> {
    const U8_MAX_IN_USIZE: usize = u8::MAX as usize;
    pub fn new(snake: Snake2D, food: Position2D) -> Self {
        if W > Self::U8_MAX_IN_USIZE || H > Self::U8_MAX_IN_USIZE {
            panic!(
                "Cannot use dimensions bigger than {}",
                Self::U8_MAX_IN_USIZE
            );
        }

        Self {
            snake,
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
