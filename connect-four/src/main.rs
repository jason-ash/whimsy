use common::{GameState, MonteCarloTree, NodeId};
use connect_four::{Checker, Game};
use nanorand::tls_rng;

fn main() {
    let mut rng = tls_rng();
    let mut tree = MonteCarloTree::<Game>::default();
    println!(
        "{:?}",
        tree.tree.get(NodeId(0)).unwrap().data().current_player()
    );

    for i in 0..100000 {
        tree.run(2.0, None, &mut rng);
    }
    let total = tree.tree.get(NodeId(0)).unwrap().data().score();
    println!("{:?}", total);

    for child in tree.tree.get(NodeId(0)).unwrap().children().into_iter() {
        let node = tree.tree.get(*child).unwrap();
        let uct = node.data().uct(&Checker::Red, 2.0, 10000);
        println!("{:?}: {}", node.data().previous_move().unwrap().1, uct);
    }
}
