use crate::{Agent, GameState};
use nanorand::{Rng, WyRand};
use petgraph::Graph;

#[derive(Debug)]
pub struct MonteCarloAgent {
    rng: WyRand,
    pub graph: Graph<MonteCarloNode, MonteCarloEdge>,
}

impl MonteCarloAgent {
    pub fn new() -> Self {
        let seed = nanorand::tls_rng().generate();
        Self::seed_from_u64(seed)
    }

    pub fn seed_from_u64(seed: u64) -> Self {
        let mut graph = Graph::default();
        graph.add_node(MonteCarloNode::default());

        Self {
            rng: WyRand::new_seed(seed),
            graph,
        }
    }

    /// find the next node to explore
    pub fn select(&self) -> Option<&MonteCarloNode> {
        todo!()
    }

    /// add nodes and edges to the graph
    pub fn expand(&mut self) {
        todo!()
    }

    pub fn backpropagate(&mut self) {
        todo!()
    }
}

impl Default for MonteCarloAgent {
    fn default() -> Self {
        Self::new()
    }
}

impl Agent for MonteCarloAgent {
    fn step(&mut self, game: &GameState) -> usize {
        // temporarily use a random index
        let open = game.open_indices().collect::<Vec<_>>();
        let idx = self.rng.generate_range(0..open.len());
        open[idx]
    }
}

#[derive(Debug)]
pub struct MonteCarloNode {
    pub state: GameState,
    pub visits: u32,
    pub score: f32,
}

impl MonteCarloNode {
    pub fn new(state: GameState) -> Self {
        Self {
            state,
            visits: 0,
            score: 0.0,
        }
    }

    pub fn rollout(&self) -> f32 {
        todo!()
    }
}

impl Default for MonteCarloNode {
    fn default() -> Self {
        Self {
            state: GameState::default(),
            visits: 0,
            score: 0.0,
        }
    }
}

#[derive(Debug)]
pub struct MonteCarloEdge {
    pub action: usize,
    pub visits: u32,
}

impl MonteCarloEdge {
    pub fn new(action: usize) -> Self {
        Self { action, visits: 0 }
    }
}
