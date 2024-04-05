use std::{collections::HashMap, hash::Hash, ops::Add};

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

    fn player_reward(&self, player: &Self::Player) -> f64 {
        self.reward()
            .get(player)
            .map(|r| (*r).into())
            .unwrap_or_default()
    }
}
