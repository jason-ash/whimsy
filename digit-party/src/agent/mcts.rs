use nanorand::{Rng, WyRand};
use petgraph::Graph;

use crate::GameState;

pub struct MonteCarloAgent {
    rng: WyRand,
    graph: Graph<GameState, ()>,
}

impl MonteCarloAgent {
    pub fn new() -> Self {
        let seed = nanorand::tls_rng().generate();
        Self::seed_from_u64(seed)
    }

    pub fn seed_from_u64(seed: u64) -> Self {
        Self {
            rng: WyRand::new_seed(seed),
            graph: Graph::default(),
        }
    }
}
