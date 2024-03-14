use nanorand::{Rng, WyRand};

#[derive(Debug)]
pub struct GameState {
    seed: u64,
    digits: [u8; 25],
    board: Board,
}

impl GameState {
    pub fn new() -> Self {
        let seed = nanorand::tls_rng().generate();
        Self::seed_from_u64(seed)
    }

    pub fn seed_from_u64(seed: u64) -> Self {
        let mut rng = WyRand::new_seed(seed);
        let mut digits = [0u8; 25];
        for i in digits.iter_mut() {
            *i = rng.generate_range(1..=9);
        }
        Self {
            seed,
            digits,
            board: Board::default(),
        }
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Default)]
pub struct Board([Option<u8>; 25]);

impl Board {
    pub fn new() -> Self {
        Self::default()
    }
}
