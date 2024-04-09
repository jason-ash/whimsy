use common::collections::MonteCarloTree;
use petgraph::Direction;
use tic_tac_toe::{Game, Player};

fn main() {
    let mut tree = MonteCarloTree::<Game>::new();
    for _ in 0..800000 {
        tree.iterate(1.2);
    }

    let root = tree.graph.node_weight(tree.root).unwrap();
    println!("{:?}", root);
    for child in tree
        .graph
        .neighbors_directed(tree.root, Direction::Outgoing)
    {
        println!("{:?}", tree.graph.node_weight(child).unwrap());
    }

    // for node in tree.graph.node_indices() {
    //     println!("{:?} : {:?}", node, tree.graph.node_weight(node).unwrap());
    // }

    // println!("{:?}", tree);
}
