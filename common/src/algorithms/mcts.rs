pub trait MonteCarloNode {
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
    type ActionIter: IntoIterator<Item = Self::GameAction>;
    type RewardIter: IntoIterator<Item = (Self::Player, f32)>;

    fn update(&self, action: Self::GameAction) -> Self;
    fn outcome(&self) -> Option<Self::RewardIter>;
    fn action_iter(&self) -> Self::ActionIter;
    fn current_player(&self) -> Self::Player;
}
