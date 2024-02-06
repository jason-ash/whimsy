pub trait MonteCarloTreeSearchNode {
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
