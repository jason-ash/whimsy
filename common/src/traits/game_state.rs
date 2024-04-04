use std::ops::Add;

pub trait GameState: Default + Clone + Sized {
    type Reward: Add<Output = Self::Reward> + Copy + Default;
    type Action: Clone;
    type Player: Clone;
    type ActionIter: Iterator<Item = (Self::Player, Self::Action)>;
    type Error;

    fn reward(&self) -> Vec<(Self::Player, Self::Reward)>;
    fn is_complete(&self) -> bool;
    fn action_iter(&self) -> Self::ActionIter;
    fn step(self, player: &Self::Player, action: &Self::Action) -> Result<Self, Self::Error>;
}
