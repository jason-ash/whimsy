use wordle::calculate_word_pairs;

fn main() {
    let result = calculate_word_pairs();
    println!("compiled {} results.", result.len());
}
