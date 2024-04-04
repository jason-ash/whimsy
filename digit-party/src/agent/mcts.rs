use crate::{
    error::{Error, Result},
    Agent, GameState,
};
use nanorand::{Rng, WyRand};
use petgraph::{
    graph::{node_index, NodeIndex},
    visit::Bfs,
    Direction::{Incoming, Outgoing},
    Graph,
};
use std::cmp::Ordering;

impl Agent for MonteCarloAgent {
    fn step(&mut self, game: &GameState) -> usize {
        todo!()
    }
}

#[derive(Debug)]
pub enum MonteCarloTimeBudget {
    Iterations(usize),
    Duration(std::time::Duration),
}

#[derive(Debug)]
pub struct MonteCarloAgent {
    pub rng: WyRand,
    pub graph: Graph<MonteCarloNode, MonteCarloEdge>,
    pub budget: MonteCarloTimeBudget,
}

impl MonteCarloAgent {
    pub fn new(budget: MonteCarloTimeBudget) -> Self {
        let seed = nanorand::tls_rng().generate();
        Self::seed_from_u64(seed, budget)
    }

    pub fn seed_from_u64(seed: u64, budget: MonteCarloTimeBudget) -> Self {
        let mut graph = Graph::default();
        graph.add_node(MonteCarloNode::default());

        Self {
            rng: WyRand::new_seed(seed),
            graph,
            budget,
        }
    }

    pub fn step(&mut self, state: &GameState) {
        let mut bfs = Bfs::new(&self.graph, node_index(0));
        todo!()
    }

    /// find the next node to explore
    pub fn select(&self, node_id: NodeIndex, c: f32) -> Result<NodeIndex> {
        todo!()
    }

    pub fn expand(&mut self, node_id: NodeIndex) {
        let state = self
            .graph
            .node_weight(node_id)
            .expect("to find a valid node.")
            .state
            .clone();

        for action in state.open_indices() {
            let state = state.step(action);
            let node_weight = MonteCarloNode::new(state);
            let edge_weight = MonteCarloEdge::new(action);
            let child_id = self.graph.add_node(node_weight);
            let _ = self.graph.add_edge(node_id, child_id, edge_weight);
        }
    }

    pub fn backpropagate(&mut self, node_id: NodeIndex, score: u32) {
        // starting at a given node, update the score and visits for all ancestors
        let node = self
            .graph
            .node_weight_mut(node_id)
            .expect("to find a valid node.");
        node.visits += 1;
        node.score += score;

        match self.graph.neighbors_directed(node_id, Incoming).next() {
            Some(parent_id) => self.backpropagate(parent_id, score),
            None => (),
        }
    }
}

impl Default for MonteCarloAgent {
    fn default() -> Self {
        let budget = MonteCarloTimeBudget::Duration(std::time::Duration::from_millis(1000));
        Self::new(budget)
    }
}

#[derive(Debug)]
pub struct MonteCarloNode {
    pub state: GameState,
    pub visits: u32,
    pub score: u32,
}

impl MonteCarloNode {
    pub fn new(state: GameState) -> Self {
        Self {
            state,
            visits: 0,
            score: 0,
        }
    }

    pub fn uct(&self, c: f32, parent_visits: u32) -> f32 {
        let n = self.visits as f32;

        if n == 0.0 {
            f32::INFINITY
        } else {
            let w = self.score as f32;
            let p = parent_visits as f32;
            w / n + c * (p.ln() / n).sqrt()
        }
    }

    /// play randomly until the end of the game, returning the final score
    pub fn rollout(&self, rng: &mut WyRand) -> u32 {
        let mut game = self.state.clone();
        loop {
            if game.is_complete() {
                break;
            }

            let actions = game.open_indices().collect::<Vec<_>>();
            let idx = rng.generate_range(0..actions.len());
            game = game.step(actions[idx]);
        }
        game.score()
    }
}

impl Default for MonteCarloNode {
    fn default() -> Self {
        Self {
            state: GameState::default(),
            visits: 0,
            score: 0,
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
