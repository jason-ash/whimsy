use common::collections::GameState;
use std::collections::HashMap;

pub mod error;

use error::Error;

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    X,
    O,
}

impl From<Player> for char {
    fn from(player: Player) -> Self {
        match player {
            Player::X => 'X',
            Player::O => 'O',
        }
    }
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::X => write!(f, "X"),
            Self::O => write!(f, "O"),
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        Self::X
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Game {
    board: [Option<Player>; 9],
    current_player: Player,
}

impl Game {
    pub fn new(board: [Option<Player>; 9], current_player: Player) -> Self {
        Self {
            board,
            current_player,
        }
    }

    pub fn open_indices(&self) -> Vec<usize> {
        self.board
            .iter()
            .enumerate()
            .filter_map(|(i, v)| if v.is_none() { Some(i) } else { None })
            .collect()
    }
}

impl GameState for Game {
    type Reward = f64;
    type Action = usize;
    type Player = Player;
    type ActionIter = std::vec::IntoIter<(Self::Player, Self::Action)>;
    type Error = Error;

    fn current_player(&self) -> Self::Player {
        self.current_player
    }

    fn reward(&self) -> HashMap<Self::Player, Self::Reward> {
        let mut winner = THREES.into_iter().filter_map(|[a, b, c]| {
            if self.board[a].is_some()
                && self.board[a] == self.board[b]
                && self.board[b] == self.board[c]
            {
                self.board[a]
            } else {
                None
            }
        });

        if let Some(player) = winner.next() {
            HashMap::from([(player, 1.0)])
        } else {
            HashMap::default()
        }
    }

    fn is_complete(&self) -> bool {
        THREES.into_iter().any(|[a, b, c]| {
            self.board[a].is_some()
                && self.board[a] == self.board[b]
                && self.board[b] == self.board[c]
        })
    }

    fn step(
        self,
        player: &Self::Player,
        action: &Self::Action,
    ) -> std::prelude::v1::Result<Self, Self::Error> {
        let mut board = self.board;
        if board[*action].is_some() {
            Err(Error::InvalidAction(player.clone(), *action))
        } else {
            board[*action] = Some(*player);
            let current_player = match self.current_player {
                Player::X => Player::O,
                Player::O => Player::X,
            };
            Ok(Self {
                board,
                current_player,
            })
        }
    }

    fn action_iter(&self) -> Self::ActionIter {
        self.open_indices()
            .into_iter()
            .map(|ix| (self.current_player, ix))
            .collect::<Vec<_>>()
            .into_iter()
    }
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            " {:1} | {:1} | {:1}",
            self.board[0].map_or_else(|| ' ', |p| p.into()),
            self.board[1].map_or_else(|| ' ', |p| p.into()),
            self.board[2].map_or_else(|| ' ', |p| p.into())
        )?;
        writeln!(f, "---+---+---")?;
        writeln!(
            f,
            " {:1} | {:1} | {:1}",
            self.board[3].map_or_else(|| ' ', |p| p.into()),
            self.board[4].map_or_else(|| ' ', |p| p.into()),
            self.board[5].map_or_else(|| ' ', |p| p.into()),
        )?;
        writeln!(f, "---+---+---")?;
        writeln!(
            f,
            " {:1} | {:1} | {:1}",
            self.board[6].map_or_else(|| ' ', |p| p.into()),
            self.board[7].map_or_else(|| ' ', |p| p.into()),
            self.board[8].map_or_else(|| ' ', |p| p.into()),
        )
    }
}

const THREES: [[usize; 3]; 8] = [
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    [0, 4, 8],
    [2, 4, 6],
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn setup_default_game() {
        let game = Game::default();
        assert_eq!(game.board, [None; 9]);
        assert_eq!(game.current_player, Player::X);
    }

    #[test]
    fn test_is_complete() {
        let board = [
            Some(Player::X),
            Some(Player::X),
            Some(Player::X),
            Some(Player::O),
            None,
            Some(Player::O),
            None,
            Some(Player::O),
            None,
        ];
        let game = Game::new(board, Player::O);
        assert!(game.is_complete());
    }
}
