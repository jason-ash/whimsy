use connect_four::{Board, Checker};

fn main() {
    let moves = &[
        (Checker::Red, 3),
        (Checker::Yellow, 3),
        (Checker::Red, 4),
        (Checker::Yellow, 5),
        (Checker::Red, 3),
        (Checker::Yellow, 6),
        (Checker::Red, 3),
        (Checker::Yellow, 3),
        (Checker::Red, 6),
        (Checker::Yellow, 0),
        (Checker::Red, 1),
        (Checker::Red, 1),
        (Checker::Red, 1),
        (Checker::Yellow, 2),
        (Checker::Yellow, 2),
        (Checker::Yellow, 2),
        (Checker::Yellow, 2),
    ];
    let board = Board::from_moves(moves).unwrap();

    println!("{}", &board);

    println!("{:?}", board.outcome());
}
