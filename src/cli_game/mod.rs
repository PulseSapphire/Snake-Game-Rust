use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use crate::engine_2d::Engine2D;
use crate::engine_2d::game_controller::GameController2D;
use crate::engine_2d::game_state::board_2d::Board2D;
use crate::game::types::position::Position2D;

struct CLIGame<'a, const W: usize, const H: usize> {
    engine: Engine2D<W, H, GameController2D<'a, SmallRng, Board2D<W, H>>>,
}

impl <'a, const W: usize, const H: usize> CLIGame<'a, W, H> {
    pub fn new () -> Self {
        let mut rng = SmallRng::from_entropy();
        let start_pos = Position2D { x: rng.gen_range(0..W) as u8, y: rng.gen_range(0..H) as u8  };
        let mut start_food_pos = Position2D { x: rng.gen_range(0..W) as u8, y: rng.gen_range(0..H) as u8  };

        while start_food_pos == start_pos {
            start_food_pos = Position2D { x: rng.gen_range(0..W) as u8, y: rng.gen_range(0..H) as u8  };
        }

        let engine = Engine2D::new(start_pos, start_food_pos);

        Self {
            engine,
        }
    }

    fn start() {

    }
}
