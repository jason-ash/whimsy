pub trait Node {
    fn rollout(&self) -> f64;

    fn score(&self) -> f64;
    fn set_score(&mut self, score: f64);

    fn visits(&self) -> usize;
    fn set_visits(&mut self, visits: usize);

    fn parent_visits(&self) -> usize;

    fn uct(&self, c: f64) -> f64 {
        if self.visits() == 0 {
            return f64::INFINITY;
        }

        let w = self.score();
        let n = self.visits() as f64;
        let p = self.parent_visits() as f64;

        w / n + c * (p.ln() / n).sqrt()
    }
}

pub mod old {
    use crate::{collections::NodeId, Tree};
    use nanorand::Rng;
    use std::{
        cmp::Ordering,
        collections::HashMap,
        fmt::Debug,
        hash::Hash,
        ops::{Add, AddAssign},
    };
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

        pub fn backpropagate(
            &mut self,
            node: NodeId,
            outcome: GameScore<T::Player, f32>,
        ) -> Option<()> {
            // given a node and a rollout outcome, propagate the value backwards to the tree root.

            let mut node_id = Some(node);

            while let Some(id) = node_id {
                let node = self.tree.get_mut(id)?;
                let score = node.data().score() + outcome.clone();

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
        type Player: Hash + Eq + PartialEq + Clone;
        type Action: Clone + PartialEq;

        fn players(&self) -> Vec<Self::Player>;
        fn actions(&self) -> Vec<(Self::Player, Self::Action)>;
        fn step(&self, player: Self::Player, action: Self::Action) -> Self;
        fn reward(&self) -> Option<GameScore<Self::Player, f32>>;

        // these are here temporarily, but probably belong at the `Node` level.
        fn score(&self) -> GameScore<Self::Player, f32>;
        fn set_score(&mut self, score: GameScore<Self::Player, f32>);
        fn visits(&self) -> u32;
        fn set_visits(&mut self, visits: u32);
        fn uct(&self, player: &Self::Player, c: f32, parent_visits: u32) -> f32 {
            let n = self.visits() as f32;
            let w = self.score().map.get(player).cloned().unwrap_or_default();
            w / n + c * ((parent_visits as f32).ln() / n).sqrt()
        }

        fn current_player(&self) -> Self::Player;
        fn previous_move(&self) -> Option<&(Self::Player, Self::Action)>;

        fn rollout<R, const OUTPUT: usize>(
            &self,
            rng: &mut R,
        ) -> Option<GameScore<Self::Player, f32>>
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

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct GameScore<T, U>
    where
        T: Hash + Eq + PartialEq + Clone,
        U: Default + Copy + Add<Output = U>,
    {
        pub map: HashMap<T, U>,
    }

    impl<T, U> Add for GameScore<T, U>
    where
        T: Hash + Eq + PartialEq + Clone,
        U: Default + Copy + Add<Output = U> + AddAssign,
    {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            let mut map = self.map.clone();
            for (key, value) in rhs.map.into_iter() {
                *map.entry(key).or_default() += value;
            }
            Self { map }
        }
    }

    impl<T, U> Default for GameScore<T, U>
    where
        T: Hash + Eq + PartialEq + Clone,
        U: Default + Copy + Add<Output = U> + AddAssign,
    {
        fn default() -> Self {
            Self {
                map: HashMap::default(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::old::*;

    #[test]
    fn game_score_add_ok() {
        let a = {
            let mut score = GameScore::<usize, f32>::default();
            score.map.insert(0, 0.0);
            score.map.insert(1, 0.5);
            score.map.insert(2, 0.75);
            score.map.insert(3, 0.25);
            score.map.insert(4, 1.0);
            score
        };
        let b = {
            let mut score = GameScore::<usize, f32>::default();
            score.map.insert(0, 0.5);
            score.map.insert(2, 0.5);
            score.map.insert(4, 0.25);
            score.map.insert(5, 0.75);
            score
        };
        let expected = {
            let mut score = GameScore::<usize, f32>::default();
            score.map.insert(0, 0.5);
            score.map.insert(1, 0.5);
            score.map.insert(2, 1.25);
            score.map.insert(3, 0.25);
            score.map.insert(4, 1.25);
            score.map.insert(5, 0.75);
            score
        };
        assert_eq!(a + b, expected);
    }
}
