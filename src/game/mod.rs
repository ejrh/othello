mod bitboard;
pub mod bitboardgame;
pub mod default;
mod direction;

use std::fmt::{Debug, Display, Formatter, Write};
use rand::prelude::SliceRandom;

use crate::game::default::DefaultBoard;
use crate::game::GameParseError::{InvalidPiece, TooManyColumns, TooManyRows};

pub type Score = i32;

pub type Pos = i8;

const BOARD_SIZE: Pos = 8;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Colour {
    Black,
    White
}

pub trait Board: Default {
    type MoveSet: IntoIterator<Item=Move>;

    fn new() -> Self {
        let mut board = Self::default();
        board.set(3, 3, Some(Colour::Black));
        board.set(3, 4, Some(Colour::White));
        board.set(4, 3, Some(Colour::White));
        board.set(4, 4, Some(Colour::Black));
        board
    }

    fn is_valid_move(&self, mov: Move) -> bool;
    fn moves(&self, for_player: Colour) -> Self::MoveSet;
    fn apply(&self, mov: Move) -> Self;
    fn get(&self, row: Pos, col: Pos) -> Option<Colour>;
    fn set(&mut self, row: Pos, col: Pos, value: Option<Colour>);
    fn scores(&self) -> (Score, Score);
}

#[derive(Clone, PartialEq)]
pub struct Game<B: Board=DefaultBoard> {
    pub next_turn: Colour,
    pub board: B,
}

pub type DefaultGame = Game<DefaultBoard>;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Move {
    pub player: Colour,
    pub row: Pos,
    pub col: Pos
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let colname = self.col as u8 + 65;
        let rowname = self.row as u8 + 49;
        f.write_char(colname as char)?;
        f.write_char(rowname as char)?;
        Ok(())
    }
}

fn out_of_range(row: Pos, col: Pos) -> bool {
    (row | col) as u8 & 0b11111000 != 0
}

impl Colour {
    pub(crate) fn opponent(self) -> Self {
        match self {
            Self::Black => Self::White,
            Self::White => Self::Black
        }
    }

    pub(crate) fn sign(self) -> Score {
        match self {
            Self::Black => 1,
            Self::White => -1
        }
    }
}

impl<B: Board> Game<B> {
    pub fn new() -> Game<B> {
        let mut board: B = Default::default();
        board.set(3, 3, Some(Colour::Black));
        board.set(3, 4, Some(Colour::White));
        board.set(4, 3, Some(Colour::White));
        board.set(4, 4, Some(Colour::Black));
        Game {
            next_turn: Colour::Black,
            board,
        }
    }

    pub fn empty() -> Game<B> {
        let board: B = Default::default();
        Game {
            next_turn: Colour::Black,
            board,
        }
    }

    pub fn valid_moves(&self, for_player: Colour) -> B::MoveSet {
        self.board.moves(for_player)
    }

    pub fn apply(&self, mov: Move) -> Self {
        Game {
            board: self.board.apply(mov),
            next_turn: self.next_turn.opponent(),
        }
    }

    pub fn get_piece(&self, row: Pos, col: Pos) -> Option<Colour> {
        self.board.get(row, col)
    }

    pub fn scores(&self) -> (Score, Score) {
        self.board.scores()
    }
}

impl<B: Board> Default for Game<B> {
    fn default() -> Self {
        Game::new()
    }
}

impl<B: Board> Debug for Game<B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in 0..8 {
            for j in 0..8 {
                let piece = self.board.get(i, j);
                f.write_str(match piece {
                    Some(Colour::Black) => "○",
                    Some(Colour::White) => "●",
                    _ => "·"
                })?;
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub enum GameParseError {
    TooManyRows,
    TooManyColumns,
    InvalidPiece,
}

impl<B: Board> TryFrom<&str> for Game<B> {
    type Error = GameParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut game: Game<B> = Game::empty();

        for (i, line) in value.split_terminator('\n').enumerate() {
            for (j, ch) in line.chars().enumerate() {
                let piece = match ch {
                    '○' => Some(Colour::Black),
                    '●' => Some(Colour::White),
                    '·' => None,
                    _ => return Err(InvalidPiece)
                };

                if i >= 8 { return Err(TooManyRows); }
                if j >= 8 { return Err(TooManyColumns); }
                game.board.set(i as Pos, j as Pos, piece);
            }
        }

        Ok(game)
    }
}

pub fn convert_board<B: Board, B2: Board>(board: &B) -> B2 {
    let mut new_board: B2 = B2::default();
    for i in 0..8 {
        for j in 0..8 {
            let piece = board.get(i, j);
            new_board.set(i, j, piece);
        }
    }
    new_board
}

pub fn convert<B: Board, B2: Board>(game: &Game<B>) -> Game<B2> {
    Game {
        next_turn: game.next_turn,
        board: convert_board(&game.board)
    }
}

pub fn random_board<B: Board>() -> B {
    const PIECE_CHOICES: [Option<Colour>; 3] = [None, Some(Colour::Black), Some(Colour::White)];

    let mut board = B::default();
    for i in 0..8 {
        for j in 0..8 {
            let random_piece = PIECE_CHOICES.choose(&mut rand::thread_rng()).unwrap();
            board.set(i, j, *random_piece);
        }
    }

    board
}
