use nanorand::{tls_rng, Rng, WyRand};
use petgraph::{graph::NodeIndex, Graph};
use std::{collections::HashMap, hash::Hash, marker::PhantomData, ops::Add};

#[derive(Debug)]
pub struct MonteCarloTree<N, G>
where
    N: MctsNode<G>,
    G: GameState,
{
    rng: WyRand,
    root: NodeIndex,
    graph: Graph<N, ()>,
    _state: PhantomData<G>,
}

impl<N, G> MonteCarloTree<N, G>
where
    N: MctsNode<G>,
    G: GameState,
{
    pub fn new() -> Self {
        let seed = tls_rng().generate();
        Self::seed_from_u64(seed)
    }

    pub fn seed_from_u64(seed: u64) -> Self {
        let rng = WyRand::new_seed(seed);
        let mut graph = Graph::new();
        let root = graph.add_node(N::default());
        Self {
            rng,
            root,
            graph,
            _state: PhantomData,
        }
    }

    pub fn select(&self, c: f64) -> NodeIndex {
        self.select_from(self.root, c)
    }

    pub fn expand(&mut self) -> Option<NodeIndex> {
        self.expand_from(self.root)
    }

    pub fn backpropagate(&mut self, result: &HashMap<G::Player, G::Reward>, leaf: NodeIndex) {
        self.backpropagate_from(result, leaf, self.root)
    }

    pub fn select_from(&self, start: NodeIndex, c: f64) -> NodeIndex {
        todo!()
    }

    pub fn expand_from(&mut self, node: NodeIndex) -> Option<NodeIndex> {
        todo!()
    }

    pub fn backpropagate_from(
        &mut self,
        result: &HashMap<G::Player, G::Reward>,
        leaf: NodeIndex,
        root: NodeIndex,
    ) {
        todo!()
    }
}

impl<N, G> Default for MonteCarloTree<N, G>
where
    N: MctsNode<G>,
    G: GameState,
{
    fn default() -> Self {
        Self::new()
    }
}

pub trait MctsNode<G: GameState>: Default {
    fn state(&self) -> &G;
    fn visits(&self) -> u32;
    fn visits_mut(&mut self) -> &mut u32;
    fn reward(&self) -> &HashMap<G::Player, G::Reward>;
    fn reward_mut(&mut self) -> &mut HashMap<G::Player, G::Reward>;

    fn uct(&self, c: f64, parent_visits: u32, player: &G::Player) -> f64 {
        let parent_visits = parent_visits as f64;
        let visits = self.visits() as f64;
        let reward = self
            .reward()
            .get(player)
            .map(|r| (*r).into())
            .unwrap_or_default();

        reward / visits + c * (parent_visits.ln() / visits).sqrt()
    }

    fn rollout(&self, rng: &mut WyRand) -> HashMap<G::Player, G::Reward> {
        todo!()
    }
}

pub trait GameState: Default + Clone + Eq + Sized {
    type Reward: Add<Output = Self::Reward> + Into<f64> + Copy + Default;
    type Action: Clone;
    type Player: Clone + Hash + Eq;
    type ActionIter: Iterator<Item = (Self::Player, Self::Action)>;
    type Error;

    fn reward(&self) -> HashMap<Self::Player, Self::Reward>;
    fn is_complete(&self) -> bool;
    fn action_iter(&self) -> Self::ActionIter;
    fn step(self, player: &Self::Player, action: &Self::Action) -> Result<Self, Self::Error>;
}
