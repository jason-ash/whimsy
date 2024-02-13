use crate::{collections::NodeId, Tree};
use nanorand::Rng;
use std::cmp::Ordering;

pub struct MonteCarloTree<T: GameState> {
    tree: Tree<T>,
}

impl<T: GameState + Clone> MonteCarloTree<T> {
    pub fn ponder(&mut self, node: NodeId, iterations: usize) {
        // run the operation for a number of iterations
        todo!()
    }

    pub fn best_action(&self, node: NodeId) -> Option<T::GameAction> {
        // select the child node that maximizes uct
        todo!()
    }

    fn select(&self, c: f32, node: NodeId) -> Option<NodeId> {
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
                    a.1.data()
                        .uct(c, parent_visits)
                        .partial_cmp(&b.1.data().uct(c, parent_visits))
                        .unwrap_or_else(|| Ordering::Equal)
                })
                .map(|(id, _)| id)?;
        }
    }

    fn expand(&mut self, node: NodeId) -> Option<()> {
        // add all nodes to the tree from this node.
        let parent = self.tree.get(node)?.data().clone();
        for child in parent
            .action_iter()
            .map(|action| parent.clone().update(action))
        {
            self.tree.insert(child, Some(node));
        }
        Some(())
    }

    fn backpropagate(&mut self, node: NodeId) {
        todo!()
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
    Self: Sized,
{
    type Player;
    type GameAction;
    type ActionIter: Iterator<Item = Self::GameAction>;
    type RewardIter: Iterator<Item = (Self::Player, f32)>;

    // these are here temporarily, but probably belong at the `Node` level.
    fn score(&self) -> f32;
    fn visits(&self) -> u32;
    fn uct(&self, c: f32, parent_visits: u32) -> f32 {
        let n = self.visits() as f32;
        self.score() / n + c * ((parent_visits as f32).ln() / n).sqrt()
    }

    fn update(self, action: Self::GameAction) -> Self;
    fn outcome(&self) -> Option<Self::RewardIter>;
    fn action_iter(&self) -> Self::ActionIter;
    fn current_player(&self) -> Self::Player;

    fn rollout<R, const OUTPUT: usize>(self, rng: &mut R) -> Option<Self::RewardIter>
    where
        R: Rng<OUTPUT>,
    {
        let mut state = self;
        loop {
            if let Some(outcome) = state.outcome() {
                return Some(outcome);
            }
            let action = {
                let mut actions = state.action_iter().collect::<Vec<_>>();
                if actions.is_empty() {
                    None
                } else {
                    let index = rng.generate_range(0..actions.len());
                    Some(actions.swap_remove(index))
                }
            }?;
            state = state.update(action);
        }
    }
}
