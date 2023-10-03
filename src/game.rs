use std::fmt::{Debug, Formatter};

type Pos = i8;

const BOARD_SIZE: Pos = 8;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Colour {
    Black,
    White
}

#[derive(Clone, Default)]
pub(crate) struct Square {
    pub(crate) piece: Option<Colour>
}

type Board = [[Square; BOARD_SIZE as usize]; BOARD_SIZE as usize];

#[derive(Clone)]
pub struct Game {
    pub next_turn: Colour,
    pub(crate) board: Board
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Move {
    pub player: Colour,
    pub row: Pos,
    pub col: Pos
}

const DIRECTIONS: &[(Pos, Pos)] = &[
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1)
];

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
}

impl Game {
    pub fn new() -> Game {
        let mut board: Board = Default::default();
        board[3][3].piece = Some(Colour::Black);
        board[3][4].piece = Some(Colour::White);
        board[4][3].piece = Some(Colour::White);
        board[4][4].piece = Some(Colour::Black);
        Game {
            next_turn: Colour::Black,
            board
        }
    }

    pub fn get_piece(&self, row: Pos, col: Pos) -> Option<Colour> {
        self.board[row as usize][col as usize].piece
    }

    fn count(&self, player: Colour, row: Pos, col: Pos, dy: Pos, dx: Pos) -> usize {
        let mut count = 0;
        let mut iter = (dy, dx).iterate_from(row, col);

        loop {
            let Some((r, c)) = iter.next() else { return 0; };
            let Some(colour) = self.board[r as usize][c as usize].piece else { return 0; };

            if colour == player { break; }
            count += 1;
        }

        count
    }

    pub(crate) fn is_valid_move(&self, mov: Move) -> bool {
        if mov.player != self.next_turn { return false; }
        if self.board[mov.row as usize][mov.col as usize].piece.is_some() { return false; }

        for (dy, dx) in DIRECTIONS {
            if self.count(mov.player, mov.row, mov.col, *dy, *dx) > 0 {
                return true;
            }
        }

        false
    }

    pub fn valid_moves(&self) -> ValidMoveIterator {
        ValidMoveIterator::new(self)
    }

    fn flip(&mut self, player: Colour, row: Pos, col: Pos, dy: Pos, dx: Pos) {
        let mut iter = (dy, dx).iterate_from(row, col);

        loop {
            let Some((r, c)) = iter.next() else { return; };
            let Some(colour) = self.board[r as usize][c as usize].piece else { return; };

            if colour == player { break; }
        }

        let mut iter = (dy, dx).iterate_from(row, col);
        loop {
            let Some((r, c)) = iter.next() else { return; };
            let Some(colour) = self.board[r as usize][c as usize].piece else { return; };

            if colour == player { break; }

            self.board[r as usize][c as usize].piece = Some(player);
        }
    }

    pub fn apply(&self, mov: Move) -> Self {
        assert!(self.is_valid_move(mov));
        let mut newgame = (*self).clone();
        newgame.next_turn = self.next_turn.opponent();

        for (dy, dx) in DIRECTIONS {
            newgame.flip(mov.player, mov.row, mov.col, *dy, *dx);
        }

        newgame.board[mov.row as usize][mov.col as usize].piece = Some(mov.player);

        newgame
    }
}

impl Debug for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.board {
            for square in row {
                f.write_str(match square.piece {
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

pub struct ValidMoveIterator<'a> {
    game: &'a Game,
    row: Pos,
    col: Pos
}

impl<'a> ValidMoveIterator<'a> {
    fn new(game: &'a Game) -> Self {
        Self { game, row: 0, col: -1 }
    }
}

impl<'a> Iterator for ValidMoveIterator<'a> {
    type Item = Move;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.col += 1;
            if self.col >= BOARD_SIZE {
                self.col = 0;
                self.row += 1;
            }
            if self.row >= BOARD_SIZE {
                self.row = 0;
                return None;
            }

            let mov = Move {
                player: self.game.next_turn,
                row: self.row,
                col: self.col,
            };
            if self.game.is_valid_move(mov) {
                return Some(mov);
            }
        }
    }
}

struct DirectionIterator {
    dx: Pos,
    dy: Pos,
    row: Pos,
    col: Pos
}

impl Iterator for DirectionIterator {
    type Item = (Pos, Pos);

    fn next(&mut self) -> Option<Self::Item> {
        self.row += self.dy;
        self.col += self.dx;
        if out_of_range(self.row, self.col) { return None; }
        Some((self.row, self.col))
    }
}

trait IterateFrom {
    fn iterate_from(&self, row: Pos, col: Pos) -> DirectionIterator;
}

impl IterateFrom for (Pos, Pos) {

    fn iterate_from(&self, row: Pos, col: Pos) -> DirectionIterator {
        DirectionIterator { dx: self.0, dy: self.1, row, col }
    }
}
