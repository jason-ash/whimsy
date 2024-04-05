use crate::traits::GameState;
use nanorand::{tls_rng, Rng, WyRand};
use petgraph::{csr::DefaultIx, Graph};
use std::{collections::HashMap, marker::PhantomData};

#[derive(Debug)]
pub struct MonteCarloTree<N, G>
where
    N: MctsNode<G>,
    G: GameState,
{
    rng: WyRand,
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
        let _root = graph.add_node(N::default());
        Self {
            rng,
            graph,
            _state: PhantomData,
        }
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
}

pub trait MctsTree<G: GameState, Ix = DefaultIx> {}
