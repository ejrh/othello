mod bitboard;
pub mod bitboardgame;
pub mod default;
mod direction;

use std::fmt::{Debug, Display, Formatter, Write};

use rand::seq::SliceRandom;

use crate::default::DefaultBoard;
use crate::GameParseError::{InvalidPiece, TooManyColumns, TooManyRows};

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

pub trait Game {
    fn next_turn(&self) -> Colour;
    fn is_valid_move(&self, mov: Move) -> bool;
    fn valid_moves(&self, for_player: Colour) -> Vec<Move>;
    fn apply_in_place(&mut self, mov: Move);
    fn get_piece(&self, row: Pos, col: Pos) -> Option<Colour>;
    fn scores(&self) -> (Score, Score);
}

#[derive(Clone, PartialEq)]
pub struct GameRepr<B: Board=DefaultBoard> {
    pub next_turn: Colour,
    pub board: B,
}

impl<B: Board> GameRepr<B> {
    pub fn empty() -> Self {
        let board: B = Default::default();
        Self {
            next_turn: Colour::Black,
            board,
        }
    }

    pub fn new() -> Self {
        let mut board: B = Default::default();
        board.set(3, 3, Some(Colour::Black));
        board.set(3, 4, Some(Colour::White));
        board.set(4, 3, Some(Colour::White));
        board.set(4, 4, Some(Colour::Black));
        Self {
            next_turn: Colour::Black,
            board,
        }
    }

    pub fn apply(&self, mov: Move) -> Self {
        Self {
            board: self.board.apply(mov),
            next_turn: self.next_turn.opponent(),
        }
    }
}

impl<B: Board> Game for GameRepr<B> {
    fn next_turn(&self) -> Colour {
        self.next_turn
    }

    fn is_valid_move(&self, mov: Move) -> bool {
        self.board.is_valid_move(mov)
    }

    fn valid_moves(&self, for_player: Colour) -> Vec<Move> {
        self.board.moves(for_player).into_iter().collect()
    }

    fn apply_in_place(&mut self, mov: Move) {
        let new_g = Self {
            board: self.board.apply(mov),
            next_turn: self.next_turn.opponent(),
        };
        *self = new_g;
    }

    fn get_piece(&self, row: Pos, col: Pos) -> Option<Colour> {
        self.board.get(row, col)
    }

    fn scores(&self) -> (Score, Score) {
        self.board.scores()
    }
}

pub type DefaultGame = GameRepr<DefaultBoard>;

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
    pub fn opponent(self) -> Self {
        match self {
            Self::Black => Self::White,
            Self::White => Self::Black
        }
    }

    pub fn sign(self) -> Score {
        match self {
            Self::Black => 1,
            Self::White => -1
        }
    }
}

impl<B: Board> Default for GameRepr<B> {
    fn default() -> Self {
        GameRepr::new()
    }
}

impl<B: Board> Debug for GameRepr<B> {
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

impl<B: Board> TryFrom<&str> for GameRepr<B> {
    type Error = GameParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut game: GameRepr<B> = GameRepr::empty();

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

pub fn convert<B: Board>(game: &dyn Game) -> GameRepr<B> {
    let mut b = DefaultBoard::new();
    for i in 0..8 {
        for j in 0..8 {
            let piece = game.get_piece(i, j);
            b.set(i, j, piece);
        }
    }
    GameRepr {
        next_turn: game.next_turn(),
        board: convert_board(&b)
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
