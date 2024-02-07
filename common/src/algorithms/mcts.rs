use crate::collections::Tree;

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
    fn select(&self, node_id: Option<usize>, c: f64) -> Option<usize>;
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

impl<T: Node> Select for Tree<T> {
    fn select(&self, node_id: Option<usize>, c: f64) -> Option<usize> {
        let node_id = node_id.unwrap_or(0);
        let parent_visits = 10;

        // TODO... abstract this into a Traverse trait on the tree itself.
        // let node_id = if self.is_leaf(node_id)? {
        //     node_id
        // } else {
        //     self.get_children_ids(node_id)?
        //         .iter()
        //         .map(|&id| {
        //             let uct = self.get_node(id)?.uct(c, parent_visits);
        //             Some((id, uct))
        //         })
        //         .flatten()
        //         .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
        //         .map(|(id, _)| id)?
        // };

        todo!()
    }
}
