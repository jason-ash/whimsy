/// implement Node on the objects stored in the tree, for example a game state, and
/// make sure it has fields like `visits` and `score` for the mcts calculations.
pub trait Node {
    fn score(&self) -> f64;
    fn visits(&self) -> u64;
    fn uct(&self, c: f64, parent_visits: u64) -> f64 {
        if self.visits() == 0 {
            f64::INFINITY
        } else {
            let w = self.score();
            let n = self.visits() as f64;
            w / n + c * ((parent_visits as f64).ln() / n).sqrt()
        }
    }
}

pub trait Select {
    fn select(&self) -> usize;
}

pub trait Expand {
    fn expand(&mut self, node_id: usize) -> usize;
}

pub trait Simulate {
    fn simulate(&self, node_id: usize) -> usize;
}

pub trait Backpropagate {
    fn backpropagate(&mut self) -> usize;
}

pub trait MonteCarloTreeSearch: Select + Expand + Simulate + Backpropagate {
    fn run(&mut self, iterations: u32);
}
