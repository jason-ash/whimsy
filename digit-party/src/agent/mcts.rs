use crate::{Agent, GameState};
use nanorand::{Rng, WyRand};
use petgraph::{
    graph::{node_index, NodeIndex},
    Direction::{Incoming, Outgoing},
    Graph,
};
use std::cmp::Ordering;

#[derive(Debug)]
pub struct MonteCarloAgent {
    pub rng: WyRand,
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
    pub fn select(&self, c: f32) -> NodeIndex {
        let mut current = node_index(0);

        loop {
            let visits = self
                .graph
                .node_weight(current)
                .map(|node| node.visits)
                .expect("to find a valid node.");

            let children = self
                .graph
                .neighbors_directed(current, Outgoing)
                .filter_map(|node_id| self.graph.node_weight(node_id).map(|node| (node_id, node)))
                .map(|(node_id, node)| (node_id, node.uct(c, visits)));

            match children.max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal)) {
                Some((child_id, _)) => current = child_id,
                None => return current,
            }
        }
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
        let mut current = node_id;

        loop {
            let node = self
                .graph
                .node_weight_mut(current)
                .expect("to find a valid node.");

            node.visits += 1;
            node.score += score;

            match self.graph.neighbors_directed(current, Incoming).next() {
                Some(parent_id) => current = parent_id,
                None => break,
            }
        }
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
        let w = self.score as f32;
        let p = parent_visits as f32;

        if n == 0.0 {
            f32::INFINITY
        } else {
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
