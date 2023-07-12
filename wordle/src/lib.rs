mod words;
pub use words::{GUESSES, SOLUTIONS};

pub fn calculate_word_pairs() -> Vec<(Word, Word, Response)> {
    let mut map = Vec::with_capacity(GUESSES.len() * SOLUTIONS.len());
    for guess in GUESSES.iter() {
        for secret in SOLUTIONS.iter() {
            map.push((
                Word::from(*guess),
                Word::from(*secret),
                evaluate_guess(*guess, *secret),
            ));
        }
    }
    map
}

#[derive(Debug, Eq, PartialEq)]
pub struct Response([Clue; 5]);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Clue {
    Correct,
    Incorrect,
    Misplaced,
}

pub struct Word([char; 5]);

impl From<&str> for Word {
    fn from(value: &str) -> Self {
        assert_eq!(value.len(), 5);
        let mut word = [' '; 5];
        for (i, c) in value.chars().enumerate() {
            word[i] = c
        }
        Word(word)
    }
}

pub fn evaluate_guess(guess: impl Into<Word>, secret: impl Into<Word>) -> Response {
    let guess = guess.into();
    let mut secret = secret.into();
    let mut response = [Clue::Incorrect; 5];

    // first pass: identify correct letters
    for (i, guessed) in guess.0.iter().enumerate() {
        if guessed == &secret.0[i] {
            secret.0[i] = '_';
            response[i] = Clue::Correct;
        }
    }

    // second pass: identify misplaced letters
    for (i, guessed) in guess.0.iter().enumerate() {
        if response[i] == Clue::Incorrect {
            if let Some(j) = secret.0.iter().position(|c| c == guessed) {
                secret.0[j] = '_';
                response[i] = Clue::Misplaced;
            }
        }
    }

    Response(response)
}

#[cfg(test)]
mod tests {
    use super::Clue::{Correct as C, Incorrect as I, Misplaced as M};
    use super::*;

    #[test]
    fn test_evaluate_guess() {
        let cases = [
            ("leave", "close", [M, I, I, I, C]),
            ("cooks", "close", [C, I, C, I, M]),
            ("cheer", "close", [C, I, M, I, I]),
            ("kills", "skill", [M, M, M, C, M]),
            ("mosso", "misos", [C, M, C, M, I]),
        ];
        for (guess, secret, response) in cases {
            assert_eq!(evaluate_guess(guess, secret), Response(response));
        }
    }
}
