use nanorand::{tls_rng, Rng, WyRand};
use petgraph::{
    graph::NodeIndex,
    Direction::{Incoming, Outgoing},
    Graph,
};
use std::{collections::HashMap, hash::Hash, marker::PhantomData, ops::Add};

#[derive(Debug, Default)]
pub struct MonteCarloTree<G: GameState> {
    rng: WyRand,
    pub root: NodeIndex,
    pub graph: Graph<MonteCarloNode<G>, ()>,
    _state: PhantomData<G>,
}

impl<G: GameState> MonteCarloTree<G> {
    pub fn new() -> Self {
        let seed = tls_rng().generate();
        Self::seed_from_u64(seed)
    }

    pub fn seed_from_u64(seed: u64) -> Self {
        let rng = WyRand::new_seed(seed);
        let mut graph = Graph::new();
        let root = graph.add_node(MonteCarloNode::default());
        Self {
            rng,
            root,
            graph,
            _state: PhantomData,
        }
    }

    pub fn uct(&self, node: NodeIndex, c: f64) -> Option<f64> {
        todo!()
    }

    pub fn iterate(&mut self, c: f64) -> Option<()> {
        self.iterate_from(self.root, c)
    }

    pub fn iterate_from(&mut self, start: NodeIndex, c: f64) -> Option<()> {
        let node = self.select_from(start, c)?;

        if self.graph.node_weight(node)?.visits() == 0 {
            let reward = self.graph.node_weight(node)?.rollout(&mut self.rng);
            let _ = self.backpropagate_from(&reward, node, start);
        } else {
            self.expand_from(node)?;
        }
        Some(())
    }

    pub fn select(&self, c: f64) -> Option<NodeIndex> {
        self.select_from(self.root, c)
    }

    pub fn expand(&mut self) -> Option<()> {
        self.expand_from(self.root)
    }

    pub fn backpropagate(
        &mut self,
        result: &HashMap<G::Player, G::Reward>,
        leaf: NodeIndex,
    ) -> Option<()> {
        self.backpropagate_from(result, leaf, self.root)
    }

    pub fn select_from(&self, start: NodeIndex, c: f64) -> Option<NodeIndex> {
        let parent_visits = self.graph.node_weight(start).map(MonteCarloNode::visits)?;

        let best_child = self
            .graph
            .neighbors_directed(start, Outgoing)
            .into_iter()
            .filter_map(|ix| self.graph.node_weight(ix).map(|node| (ix, node)))
            .map(|(ix, node)| {
                let uct = node.uct(c, parent_visits);
                (ix, uct)
            })
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(ix, _)| ix);

        match best_child {
            None => Some(start),
            Some(ix) => self.select_from(ix, c),
        }
    }

    pub fn expand_from(&mut self, node: NodeIndex) -> Option<()> {
        if self.graph.neighbors_directed(node, Outgoing).count() > 0 {
            return None;
        } else {
            // generate all possible states from the current node
            let state = self.graph.node_weight(node)?.state();
            let nodes = state
                .action_iter()
                .filter_map(|(player, action)| {
                    state
                        .clone()
                        .step(&player, &action)
                        .map(MonteCarloNode::from_state)
                        .ok()
                })
                .collect::<Vec<_>>();

            for n in nodes {
                let ix = self.graph.add_node(n);
                let _ = self.graph.add_edge(node, ix, ());
            }

            Some(())
        }
    }

    pub fn backpropagate_from(
        &mut self,
        result: &HashMap<G::Player, G::Reward>,
        leaf: NodeIndex,
        root: NodeIndex,
    ) -> Option<()> {
        // is there a path from the leaf to the root?
        let mut path = Vec::new();
        path.push(leaf);
        while let Some(ix) = self
            .graph
            .neighbors_directed(*path.last().unwrap(), Incoming)
            .next()
        {
            path.push(ix);
        }

        if let Some(ix) = path.last() {
            if *ix != root {
                return None;
            }
        }

        // update the reward hashmap at each node in the path
        for ix in path.into_iter() {
            let node = self.graph.node_weight_mut(ix)?;
            *node.visits_mut() += 1;

            // add the score from the current result to the node's reward hashmap
            for (player, reward) in result.iter() {
                let entry = node
                    .reward_mut()
                    .entry(player.clone())
                    .or_insert_with(Default::default);
                *entry = *entry + *reward;
            }
        }

        Some(())
    }
}

#[derive(Debug, Default)]
pub struct MonteCarloNode<G: GameState> {
    state: G,
    visits: u32,
    reward: HashMap<G::Player, G::Reward>,
}

impl<G: GameState> MctsNode<G> for MonteCarloNode<G> {
    fn from_state(state: G) -> Self {
        Self {
            state,
            visits: 0,
            reward: HashMap::default(),
        }
    }

    fn state(&self) -> &G {
        &self.state
    }

    fn visits(&self) -> u32 {
        self.visits
    }

    fn visits_mut(&mut self) -> &mut u32 {
        &mut self.visits
    }

    fn reward(&self) -> &HashMap<G::Player, G::Reward> {
        &self.reward
    }

    fn reward_mut(&mut self) -> &mut HashMap<G::Player, G::Reward> {
        &mut self.reward
    }
}

pub trait MctsNode<G: GameState>: Default {
    fn from_state(state: G) -> Self;
    fn state(&self) -> &G;
    fn visits(&self) -> u32;
    fn visits_mut(&mut self) -> &mut u32;
    fn reward(&self) -> &HashMap<G::Player, G::Reward>;
    fn reward_mut(&mut self) -> &mut HashMap<G::Player, G::Reward>;

    fn uct(&self, c: f64, parent_visits: u32) -> f64 {
        let parent_visits = parent_visits as f64;
        let visits = self.visits() as f64;
        let reward = self
            .reward()
            .get(&self.state().current_player())
            .map(|r| (*r).into())
            .unwrap_or_default();

        if visits == 0.0 {
            f64::INFINITY
        } else {
            reward / visits + c * (parent_visits.ln() / visits).sqrt()
        }
    }

    fn rollout(&self, rng: &mut WyRand) -> HashMap<G::Player, G::Reward> {
        let mut state = self.state().clone();

        loop {
            if state.is_complete() {
                return state.reward();
            }

            let (player, action) = {
                let actions = state.action_iter().collect::<Vec<_>>();
                if actions.is_empty() {
                    return HashMap::new();
                }
                let ix = rng.generate_range(0..actions.len());
                actions[ix].clone()
            };

            state = match state.step(&player, &action) {
                Ok(s) => s,
                Err(_) => return HashMap::new(),
            };
        }
    }
}

pub trait GameState: std::fmt::Debug + Default + Clone + PartialEq + Eq + Sized {
    type Reward: Add<Output = Self::Reward> + Into<f64> + Copy + Default + std::fmt::Debug;
    type Action: Clone;
    type Player: Clone + Hash + Eq + Default + std::fmt::Debug;
    type ActionIter: Iterator<Item = (Self::Player, Self::Action)>;
    type Error;

    fn current_player(&self) -> Self::Player;
    fn reward(&self) -> HashMap<Self::Player, Self::Reward>;
    fn is_complete(&self) -> bool;
    fn action_iter(&self) -> Self::ActionIter;
    fn step(self, player: &Self::Player, action: &Self::Action) -> Result<Self, Self::Error>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Eq, PartialEq)]
    struct MockGame {
        map: HashMap<i32, i32>,
        goal: i32,
    }

    impl MockGame {
        fn new(goal: i32) -> Self {
            Self {
                map: HashMap::default(),
                goal,
            }
        }
    }

    impl Default for MockGame {
        fn default() -> Self {
            Self::new(5)
        }
    }

    impl GameState for MockGame {
        type Reward = i32;
        type Action = i32;
        type Player = i32;
        type ActionIter = std::vec::IntoIter<(Self::Player, Self::Action)>;
        type Error = ();

        fn current_player(&self) -> Self::Player {
            0
        }

        fn reward(&self) -> HashMap<Self::Player, Self::Reward> {
            self.map.clone()
        }

        fn is_complete(&self) -> bool {
            self.map.get(&0) == Some(&self.goal)
        }

        fn action_iter(&self) -> Self::ActionIter {
            vec![(0, 1), (0, -1)].into_iter()
        }

        fn step(self, player: &Self::Player, action: &Self::Action) -> Result<Self, Self::Error> {
            let mut map = self.map;
            let entry = map.entry(*player).or_insert(0);
            *entry += action;
            Ok(Self {
                map,
                goal: self.goal,
            })
        }
    }

    #[test]
    fn build_default_tree() {
        let tree = MonteCarloTree::<MockGame>::default();
        let root = tree.graph.node_weight(tree.root).unwrap();

        assert_eq!(tree.graph.node_count(), 1);
        assert_eq!(root.visits, 0);
        assert_eq!(root.reward, HashMap::default());
        assert_eq!(root.state, MockGame::default());
    }

    #[test]
    fn expand_default_tree() {
        let mut tree = MonteCarloTree::<MockGame>::default();
        tree.expand().unwrap();
        assert_eq!(tree.graph.node_count(), 3);
        let node = tree.select(2.0).unwrap();
        tree.expand_from(node).unwrap();
        assert_eq!(tree.graph.node_count(), 5);
    }
}
