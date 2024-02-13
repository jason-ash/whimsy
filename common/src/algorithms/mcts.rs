use nanorand::Rng;

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
