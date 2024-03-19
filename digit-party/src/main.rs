use digit_party::{Agent, RandomAgent};

fn main() {
    let mut agent = RandomAgent::default();

    let games = (0..10_000).map(|_| agent.play()).collect::<Vec<_>>();
    let scores = games.iter().map(|game| game.score()).collect::<Vec<_>>();

    let average = scores.iter().fold((0, 0), |mut total, score| {
        total.0 += score;
        total.1 += 1;
        total
    });

    println!("{:?}", (average.0 as f64) / (average.1 as f64));
}
