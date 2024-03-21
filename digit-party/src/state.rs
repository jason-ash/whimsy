use nanorand::{Rng, WyRand};

#[derive(Debug, Clone)]
pub struct GameState {
    pub seed: u64,
    turn: usize,
    digits: [u8; 25],
    board: [Option<u8>; 25],
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
            turn: 0,
            digits,
            board: [None; 25],
        }
    }

    pub fn score(&self) -> u32 {
        CONNECTIONS
            .map(|(a, b)| match (self.board[a], self.board[b]) {
                (Some(x), Some(y)) if x == y => x as u32,
                _ => 0,
            })
            .into_iter()
            .sum()
    }

    pub fn is_complete(&self) -> bool {
        self.board.iter().all(Option::is_some)
    }

    pub fn digit_current(&self) -> u8 {
        self.digits[self.turn]
    }

    pub fn digit_next(&self) -> Option<u8> {
        self.digits.get(self.turn + 1).cloned()
    }

    pub fn step(&self, idx: usize) -> Self {
        let mut out = self.clone();
        out.turn += 1;
        out.board[idx] = Some(self.digit_current());
        out
    }

    pub fn open_indices(&self) -> impl Iterator<Item = usize> + '_ {
        self.board
            .iter()
            .enumerate()
            .filter_map(|(i, x)| if x.is_none() { Some(i) } else { None })
    }

    pub fn child_states(&self) -> impl Iterator<Item = Self> + '_ {
        self.open_indices().map(|idx| self.step(idx))
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "+-------------------+\n")?;
        write!(
            f,
            "| {} | {} | {} | {} | {} |\n",
            self.board[0]
                .map(|x| x.to_string())
                .unwrap_or(String::from(" ")),
            self.board[1]
                .map(|x| x.to_string())
                .unwrap_or(String::from(" ")),
            self.board[2]
                .map(|x| x.to_string())
                .unwrap_or(String::from(" ")),
            self.board[3]
                .map(|x| x.to_string())
                .unwrap_or(String::from(" ")),
            self.board[4]
                .map(|x| x.to_string())
                .unwrap_or(String::from(" ")),
        )?;
        write!(
            f,
            "| {} | {} | {} | {} | {} |\n",
            self.board[5]
                .map(|x| x.to_string())
                .unwrap_or(String::from(" ")),
            self.board[6]
                .map(|x| x.to_string())
                .unwrap_or(String::from(" ")),
            self.board[7]
                .map(|x| x.to_string())
                .unwrap_or(String::from(" ")),
            self.board[8]
                .map(|x| x.to_string())
                .unwrap_or(String::from(" ")),
            self.board[9]
                .map(|x| x.to_string())
                .unwrap_or(String::from(" ")),
        )?;
        write!(
            f,
            "| {} | {} | {} | {} | {} |\n",
            self.board[10]
                .map(|x| x.to_string())
                .unwrap_or(String::from(" ")),
            self.board[11]
                .map(|x| x.to_string())
                .unwrap_or(String::from(" ")),
            self.board[12]
                .map(|x| x.to_string())
                .unwrap_or(String::from(" ")),
            self.board[13]
                .map(|x| x.to_string())
                .unwrap_or(String::from(" ")),
            self.board[14]
                .map(|x| x.to_string())
                .unwrap_or(String::from(" ")),
        )?;
        write!(
            f,
            "| {} | {} | {} | {} | {} |\n",
            self.board[15]
                .map(|x| x.to_string())
                .unwrap_or(String::from(" ")),
            self.board[16]
                .map(|x| x.to_string())
                .unwrap_or(String::from(" ")),
            self.board[17]
                .map(|x| x.to_string())
                .unwrap_or(String::from(" ")),
            self.board[18]
                .map(|x| x.to_string())
                .unwrap_or(String::from(" ")),
            self.board[19]
                .map(|x| x.to_string())
                .unwrap_or(String::from(" ")),
        )?;
        write!(
            f,
            "| {} | {} | {} | {} | {} |\n",
            self.board[20]
                .map(|x| x.to_string())
                .unwrap_or(String::from(" ")),
            self.board[21]
                .map(|x| x.to_string())
                .unwrap_or(String::from(" ")),
            self.board[22]
                .map(|x| x.to_string())
                .unwrap_or(String::from(" ")),
            self.board[23]
                .map(|x| x.to_string())
                .unwrap_or(String::from(" ")),
            self.board[24]
                .map(|x| x.to_string())
                .unwrap_or(String::from(" ")),
        )?;
        write!(f, "+-------------------+\n")
    }
}

/// Connections between points on the board, where the board is labeled 0-24 from upper left to
/// lower right.
const CONNECTIONS: [(usize, usize); 72] = [
    (0, 1),
    (1, 2),
    (2, 3),
    (3, 4),
    (5, 6),
    (6, 7),
    (7, 8),
    (8, 9),
    (10, 11),
    (11, 12),
    (12, 13),
    (13, 14),
    (15, 16),
    (16, 17),
    (17, 18),
    (18, 19),
    (20, 21),
    (21, 22),
    (22, 23),
    (23, 24),
    (0, 5),
    (5, 10),
    (10, 15),
    (15, 20),
    (1, 6),
    (6, 11),
    (11, 16),
    (16, 21),
    (2, 7),
    (7, 12),
    (12, 17),
    (17, 22),
    (3, 8),
    (8, 13),
    (13, 18),
    (18, 23),
    (4, 9),
    (9, 14),
    (14, 19),
    (19, 24),
    (0, 6),
    (1, 7),
    (2, 8),
    (3, 9),
    (5, 11),
    (6, 12),
    (7, 13),
    (8, 14),
    (10, 16),
    (11, 17),
    (12, 18),
    (13, 19),
    (15, 21),
    (16, 22),
    (17, 23),
    (18, 24),
    (1, 5),
    (2, 6),
    (3, 7),
    (4, 8),
    (6, 10),
    (7, 11),
    (8, 12),
    (9, 13),
    (11, 15),
    (12, 16),
    (13, 17),
    (14, 18),
    (16, 20),
    (17, 21),
    (18, 22),
    (19, 23),
];
