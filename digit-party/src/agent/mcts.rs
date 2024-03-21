use crate::{Agent, GameState};
use nanorand::{Rng, WyRand};
use petgraph::{
    graph::{node_index, NodeIndex},
    Direction::Outgoing,
    Graph,
};

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
    pub fn select(&self) -> NodeIndex {
        let mut current = node_index(0);
        let parent_visits = 3;
        while let Some(node) = self
            .graph
            .neighbors_directed(current, Outgoing)
            .max_by(|a, b| {
                let a = self.graph.node_weight(*a).unwrap();
                let b = self.graph.node_weight(*b).unwrap();
                a.uct(2.0, parent_visits)
                    .partial_cmp(&b.uct(2.0, parent_visits))
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
        {
            current = node;
        }
        current
    }

    /// add nodes and edges to the graph
    pub fn expand(&mut self, node_id: NodeIndex) -> Option<()> {
        let state = self
            .graph
            .node_weight(node_id)
            .map(|node| node.state.clone())?;

        for action in state.open_indices() {
            let state = state.step(action);
            let node_weight = MonteCarloNode::new(state);
            let edge_weight = MonteCarloEdge::new(action);

            let child_id = self.graph.add_node(node_weight);
            let _ = self.graph.add_edge(node_id, child_id, edge_weight);
        }

        Some(())
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
        nanorand::tls_rng().generate_range(0..100) as f32
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
