use digit_party::{Agent, MonteCarloAgent};

fn main() {
    let mut agent = MonteCarloAgent::default();
    println!("{:?}", agent);

    let games = agent.play_many(10_000);

    let scores = games.iter().map(|game| game.score()).collect::<Vec<_>>();
    let average = scores.iter().fold((0, 0), |mut total, score| {
        total.0 += score;
        total.1 += 1;
        total
    });

    println!("{:?}", (average.0 as f64) / (average.1 as f64));
}
