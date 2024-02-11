use std::fmt::{Debug, Display};

use nanorand::Rng;

/// the Connect Four board. contains cells numbered 0 through 41.
/// |  0 |  1 |  2 |  3 |  4 |  5 |  6 |
/// |  7 |  8 |  9 | 10 | 11 | 12 | 13 |
/// | 14 | 15 | 16 | 17 | 18 | 19 | 20 |
/// | 21 | 22 | 23 | 24 | 25 | 26 | 27 |
/// | 28 | 29 | 30 | 31 | 32 | 33 | 34 |
/// | 35 | 36 | 37 | 38 | 39 | 40 | 41 |
/// ====================================
#[derive(Debug)]
pub struct Board<T> {
    cells: [T; 42],
}

impl Board<Option<Checker>> {
    pub fn current_player(&self) -> Option<Checker> {
        // who has fewer pieces?
        let totals = self.cells.iter().fold((0, 0), |total, cell| match cell {
            Some(Checker::Red) => (total.0 + 1, total.1),
            Some(Checker::Yellow) => (total.0, total.1 + 1),
            None => total,
        });

        if totals.0 == totals.1 {
            Some(Checker::Red)
        } else {
            Some(Checker::Yellow)
        }
    }

    pub fn play_random(&self) -> Option<Self> {
        let moves = self.available_moves();
        if moves.is_empty() {
            None
        } else {
            let idx = nanorand::tls_rng().generate_range(0..moves.len());
            if let Some(checker) = self.current_player() {
                self.play_move(checker, moves[idx])
            } else {
                None
            }
        }
    }
}

impl<T: Debug + Clone + PartialEq> Board<Option<T>> {
    pub fn from_moves(moves: &[(T, usize)]) -> Option<Self> {
        let mut board = Self::default();

        for (item, idx) in moves {
            board = board.play_move(item.clone(), *idx)?;
        }
        Some(board)
    }

    pub fn play_move(&self, item: T, idx: usize) -> Option<Self> {
        self.next_available(idx).map(|idx| {
            let mut cells = self.cells.clone();
            cells[idx] = Some(item);
            Self { cells }
        })
    }

    pub fn available_moves(&self) -> Vec<usize> {
        (0..7).filter(|&idx| self.cells[idx].is_none()).collect()
    }

    pub fn outcome(&self) -> Option<Outcome<T>> {
        FOURS.into_iter().find_map(|indices| {
            let first = self.cells[indices[0]].as_ref()?;
            if indices
                .into_iter()
                .map(|idx| self.cells[idx].as_ref())
                .all(|cell| cell == Some(first))
            {
                Some(Outcome::Win(first.clone()))
            } else {
                None
            }
        })
    }

    fn next_available(&self, idx: usize) -> Option<usize> {
        let end = idx + 7 * 5 + 1;
        (idx..end)
            .step_by(7)
            .take(6)
            .rev()
            .find(|&idx| self.cells[idx].is_none())
    }
}

#[derive(Debug)]
pub enum Outcome<T> {
    Win(T),
    Loss(T),
    Tie,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Checker {
    Red,
    Yellow,
}

impl std::fmt::Display for Checker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Red => write!(f, "x"),
            Self::Yellow => write!(f, "o"),
        }
    }
}

impl<T: Display> Display for Board<Option<T>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..6 {
            write!(f, "|")?;
            for col in 0..7 {
                let idx = row * 7 + col;
                let s = self.cells[idx]
                    .as_ref()
                    .map(|cell| cell.to_string())
                    .unwrap_or(" ".to_string());
                write!(f, " {} |", s)?;
            }
            writeln!(f)?;
        }
        writeln!(f, "=============================")?;
        Ok(())
    }
}

impl<T: Debug + Default> Default for Board<T> {
    fn default() -> Self {
        let cells = std::iter::repeat_with(T::default)
            .take(42)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        Self { cells }
    }
}

const FOURS: [[usize; 4]; 69] = [
    [0, 1, 2, 3],
    [1, 2, 3, 4],
    [2, 3, 4, 5],
    [3, 4, 5, 6],
    [7, 8, 9, 10],
    [8, 9, 10, 11],
    [9, 10, 11, 12],
    [10, 11, 12, 13],
    [14, 15, 16, 17],
    [15, 16, 17, 18],
    [16, 17, 18, 19],
    [17, 18, 19, 20],
    [21, 22, 23, 24],
    [22, 23, 24, 25],
    [23, 24, 25, 26],
    [24, 25, 26, 27],
    [28, 29, 30, 31],
    [29, 30, 31, 32],
    [30, 31, 32, 33],
    [31, 32, 33, 34],
    [35, 36, 37, 38],
    [36, 37, 38, 39],
    [37, 38, 39, 40],
    [38, 39, 40, 41],
    [0, 7, 14, 21],
    [7, 14, 21, 28],
    [14, 21, 28, 35],
    [1, 8, 15, 22],
    [8, 15, 22, 29],
    [15, 22, 29, 36],
    [2, 9, 16, 23],
    [9, 16, 23, 30],
    [16, 23, 30, 37],
    [3, 10, 17, 24],
    [10, 17, 24, 31],
    [17, 24, 31, 38],
    [4, 11, 18, 25],
    [11, 18, 25, 32],
    [18, 25, 32, 39],
    [5, 12, 19, 26],
    [12, 19, 26, 33],
    [19, 26, 33, 40],
    [6, 13, 20, 27],
    [13, 20, 27, 34],
    [20, 27, 34, 41],
    [0, 8, 16, 24],
    [1, 9, 17, 25],
    [2, 10, 18, 26],
    [3, 11, 19, 27],
    [7, 15, 23, 31],
    [8, 16, 24, 32],
    [9, 17, 25, 33],
    [10, 18, 26, 34],
    [14, 22, 30, 38],
    [15, 23, 31, 39],
    [16, 24, 32, 40],
    [17, 25, 33, 41],
    [3, 9, 15, 21],
    [4, 10, 16, 22],
    [5, 11, 17, 23],
    [6, 12, 18, 24],
    [10, 16, 22, 28],
    [11, 17, 23, 29],
    [12, 18, 24, 30],
    [13, 19, 25, 31],
    [17, 23, 29, 35],
    [18, 24, 30, 36],
    [19, 25, 31, 37],
    [20, 26, 32, 38],
];
