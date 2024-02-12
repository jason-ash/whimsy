use common::Tree;
use connect_four::{Board, Checker};

fn main() {
    let mut tree = Tree::<u8>::default();
    let node0 = tree.insert(0, None);
    let node1 = tree.insert(1, node0);
    let node2 = tree.insert(2, node0);
    let _node3 = tree.insert(3, node1);
    let _node4 = tree.insert(4, node1);
    let node5 = tree.insert(5, node2);
    let node6 = tree.insert(6, node2);
    let _node7 = tree.insert(7, node5);
    let _node8 = tree.insert(8, node6);
    let node9 = tree.insert(9, node6);

    println!("{:?}", tree);

    // let moves = &[
    //     (Checker::Red, 3),
    //     (Checker::Yellow, 3),
    //     (Checker::Red, 4),
    //     (Checker::Yellow, 5),
    //     (Checker::Red, 3),
    //     (Checker::Yellow, 6),
    //     (Checker::Red, 3),
    //     (Checker::Yellow, 3),
    //     (Checker::Red, 6),
    //     (Checker::Yellow, 0),
    //     (Checker::Red, 1),
    //     (Checker::Red, 1),
    //     (Checker::Red, 1),
    //     (Checker::Yellow, 2),
    //     (Checker::Yellow, 2),
    //     (Checker::Yellow, 2),
    //     (Checker::Yellow, 2),
    // ];
    // let board = Board::from_moves(moves).unwrap();
    //
    // println!("{}", &board);
    //
    // println!("{:?}", board.outcome());
}
