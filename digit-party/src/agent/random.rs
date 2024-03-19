use nanorand::{Rng, WyRand};

use crate::{Agent, GameState};

pub struct RandomAgent {
    rng: WyRand,
}

impl RandomAgent {
    pub fn new() -> Self {
        let seed = nanorand::tls_rng().generate();
        Self::seed_from_u64(seed)
    }

    pub fn seed_from_u64(seed: u64) -> Self {
        Self {
            rng: WyRand::new_seed(seed),
        }
    }
}

impl Default for RandomAgent {
    fn default() -> Self {
        Self::new()
    }
}

impl Agent for RandomAgent {
    fn step(&mut self, game: &GameState) -> usize {
        let open = game.open_indices().collect::<Vec<_>>();
        let idx = self.rng.generate_range(0..open.len());
        open[idx]
    }
}
