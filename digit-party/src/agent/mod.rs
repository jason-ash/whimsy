mod mcts;
mod random;

pub use mcts::MonteCarloAgent;
pub use random::RandomAgent;

use crate::GameState;

pub trait Agent {
    fn step(&mut self, game: &GameState) -> usize;

    fn play(&mut self) -> GameState {
        let mut game = GameState::default();
        loop {
            if game.is_complete() {
                break;
            }

            let idx = Self::step(self, &game);
            game = game.step(idx);
        }

        game
    }

    fn play_many(&mut self, n: usize) -> Vec<GameState> {
        (0..n).map(|_| self.play()).collect()
    }
}
