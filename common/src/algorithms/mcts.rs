use crate::{collections::NodeId, Tree};
use nanorand::Rng;
use std::{cmp::Ordering, collections::HashMap, fmt::Debug};

pub struct MonteCarloTree<T: GameState> {
    pub tree: Tree<T>,
}

impl<T: GameState + Default> Default for MonteCarloTree<T> {
    fn default() -> Self {
        let mut tree = Tree::<T>::default();
        tree.insert(T::default(), None);
        Self { tree }
    }
}

impl<T: GameState + Debug> Debug for MonteCarloTree<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.tree)
    }
}

impl<T: GameState + Clone> MonteCarloTree<T> {
    pub fn run<R, const OUTPUT: usize>(
        &mut self,
        c: f32,
        node: Option<NodeId>,
        rng: &mut R,
    ) -> Option<()>
    where
        R: Rng<OUTPUT>,
    {
        // select
        let node_id = self.select(c, node.unwrap_or(NodeId(0)))?;
        let node = self.tree.get(node_id)?;

        // if we have already visited this node, then we expand the tree, then
        // choose the first child node that we generated to rollout.
        if node.data().visits() > 0 {
            self.expand(node_id)?;
        }

        // "reselect", which basically will grab a child of the one we selected earlier.
        let node_id = self.select(c, node_id)?;
        let node = self.tree.get(node_id)?;

        // rollout
        let outcome = node.data().clone().rollout(rng)?;

        // backprop - take the score from the rollout and populate it backwards to the root.
        self.backpropagate(node_id, outcome);

        Some(())
    }

    pub fn ponder(&mut self, node: Option<NodeId>, iterations: usize) {
        todo!()
    }

    pub fn best_action(&self, node: NodeId) -> Option<T::Action> {
        // select the child node that maximizes uct
        todo!()
    }

    pub fn select(&self, c: f32, node: NodeId) -> Option<NodeId> {
        // depth-first seach through the tree, maximizing the UCT until we hit a leaf node.
        let mut current = node;
        loop {
            let node = self.tree.get(current)?;
            if node.children().is_empty() {
                return Some(current);
            }
            let parent_visits = node.data().visits();
            current = node
                .children()
                .into_iter()
                .filter_map(|&id| self.tree.get(id).map(|node| (id, node)))
                .max_by(move |a, b| {
                    let player = a.1.data().current_player();
                    a.1.data()
                        .uct(&player, c, parent_visits)
                        .partial_cmp(&b.1.data().uct(&player, c, parent_visits))
                        .unwrap_or_else(|| Ordering::Equal)
                })
                .map(|(id, _)| id)?;
        }
    }

    pub fn expand(&mut self, node: NodeId) -> Option<()> {
        // add all nodes to the tree from this node.
        let parent = self.tree.get(node)?.data().clone();
        for child in parent
            .actions()
            .into_iter()
            .map(|(player, action)| parent.clone().step(player, action))
        {
            self.tree.insert(child, Some(node));
        }
        Some(())
    }

    pub fn backpropagate(&mut self, node: NodeId, outcome: HashMap<T::Player, f32>) -> Option<()> {
        // given a node and a rollout outcome, propagate the value backwards to the tree root.

        let mut node_id = Some(node);

        while let Some(id) = node_id {
            let node = self.tree.get_mut(id)?;

            let score = node
                .data()
                .score()
                .into_iter()
                .map(|(p1, s1)| {
                    let s2 = outcome
                        .iter()
                        .find_map(|(p2, s2)| if &p1 == p2 { Some(s2) } else { None })
                        .unwrap_or(&0.0);
                    (p1, s1 + s2)
                })
                .collect::<Vec<_>>();

            // set score and visits
            node.data_mut().set_score(score);
            let current_visits = node.data().visits();
            node.data_mut().set_visits(current_visits + 1);

            node_id = node.parent();
        }

        Some(())
    }
}

pub trait MonteCarloNode<T: GameState> {
    fn state(&self) -> &T;
    fn score(&self) -> f32;
    fn visits(&self) -> u32;
    fn uct(&self, c: f32, parent_visits: u32) -> f32 {
        let n = self.visits() as f32;
        self.score() / n + c * ((parent_visits as f32).ln() / n).sqrt()
    }
}

pub trait GameState
where
    Self: Sized + Clone,
{
    type Player: Clone + PartialEq;
    type Action: Clone + PartialEq;

    fn players(&self) -> Vec<Self::Player>;
    fn actions(&self) -> Vec<(Self::Player, Self::Action)>;
    fn step(&self, player: Self::Player, action: Self::Action) -> Self;
    fn reward(&self) -> Option<HashMap<Self::Player, f32>>;

    // these are here temporarily, but probably belong at the `Node` level.
    fn score(&self) -> Vec<(Self::Player, f32)>;
    fn set_score(&mut self, score: Vec<(Self::Player, f32)>);
    fn visits(&self) -> u32;
    fn set_visits(&mut self, visits: u32);
    fn uct(&self, player: &Self::Player, c: f32, parent_visits: u32) -> f32 {
        let n = self.visits() as f32;
        let w = self
            .score()
            .into_iter()
            .find(|(p, _)| p == player)
            .map(|(_, s)| s)
            .unwrap_or_default();
        w / n + c * ((parent_visits as f32).ln() / n).sqrt()
    }

    // fn update(self, action: Self::Action) -> Self;
    fn outcome(&self) -> Option<Vec<(Self::Player, f32)>>;
    fn current_player(&self) -> Self::Player;
    fn previous_move(&self) -> Option<&(Self::Player, Self::Action)>;

    fn rollout<R, const OUTPUT: usize>(&self, rng: &mut R) -> Option<HashMap<Self::Player, f32>>
    where
        R: Rng<OUTPUT>,
    {
        let mut state = self.clone();
        loop {
            if let Some(scores) = state.reward() {
                return Some(scores);
            }

            let (player, action) = {
                let mut actions = self.actions();
                if actions.is_empty() {
                    None
                } else {
                    let index = rng.generate_range(0..actions.len());
                    Some(actions.swap_remove(index))
                }
            }?;
            state = state.step(player, action)
        }
    }
}
