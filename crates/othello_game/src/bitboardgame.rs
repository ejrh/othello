use std::fmt::{Debug};

use crate::{bitboard, Board, Colour, Move, Pos, Score};
use crate::bitboard::{BitBoard, dumb7fill_occluded, ShiftDir};

#[derive(Clone, Debug, Default)]
pub struct BitBoardBoard {
    blacks: BitBoard,
    whites: BitBoard,
}

fn moves_in_dir(mine: BitBoard, theirs: BitBoard, shift_dir: ShiftDir) -> BitBoard {
    let moves = dumb7fill_occluded(mine, theirs, shift_dir.shift());
    let moves = moves.shift(shift_dir.shift());
    moves
}

impl Board for BitBoardBoard {
    type MoveSet = Moves;

    fn is_valid_move(&self, mov: Move) -> bool {
        todo!()
    }

    fn moves(&self, player: Colour) -> Self::MoveSet {
        let (mine, theirs) = match player {
            Colour::Black => (self.blacks, self.whites),
            Colour::White => (self.whites, self.blacks)
        };

        // macro_rules! do_multiple {
        //     (self.$f:ident($arg1:expr, $arg2:expr); $( $param:expr ),*) => {
        //         $(
        //             $f($arg1, $arg2, $param) |
        //         )* 0
        //     }
        // }
        // let moves = do_multiple!(self.moves_in_dir(mine, theirs);
        // -9, -8, -7, -1, 1, 7, 8, 9);
        let mut moves = BitBoard::new();
        for dir in bitboard::SHIFT_DIRS {
            moves |= moves_in_dir(mine, theirs, *dir);
        }
        let moves = moves & !mine & !theirs;
        Moves(player, moves)
    }

    fn apply(&self, mov: Move) -> Self {
        todo!()
    }

    fn get(&self, row: Pos, col: Pos) -> Option<Colour> {
        let b = self.blacks.bit(row, col);
        let w = self.whites.bit(row, col);
        if b { Some(Colour::Black) } else if w { Some(Colour::White) } else { None }
    }

    fn set(&mut self, row: Pos, col: Pos, value: Option<Colour>) {
        let bit = BitBoard::from((row, col));
        self.blacks &= !bit;
        self.whites &= !bit;
        match value {
            Some(Colour::Black) => self.blacks |= bit,
            Some(Colour::White) => self.whites |= bit,
            None => ()
        };
    }

    fn scores(&self) -> (Score, Score) {
        todo!()
    }
}

#[derive(Debug)]
pub struct Moves(Colour, BitBoard);

impl Iterator for Moves {
    type Item = Move;

    fn next(&mut self) -> Option<Self::Item> {
        let nb = self.1.pop_next_bit();
        if nb.is_empty() {
            None
        } else {
            let (row, col) = nb.to_bit_pos();
            Some(Move{
                player: self.0,
                row,
                col,
            })
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Colour::Black;
    use crate::default::DefaultBoard;
    use crate::{convert_board, DefaultGame, random_board};
    use super::*;

    #[test]
    fn test_new() {
        let bb = BitBoardBoard::new();

        assert_eq!(2, bb.blacks.count());
        assert_eq!(2, bb.whites.count());
    }

    #[test]
    fn test_new_moves() {
        let bb = BitBoardBoard::new();
        let mut moves = bb.moves(Colour::Black);
        let expected_moves = BitBoard::from(&[(2, 4), (3, 5), (4, 2), (5, 3)]);
        assert_eq!(expected_moves, moves.1);
        let all_moves: Vec<_> = moves.collect();
        assert_eq!(4, all_moves.len());
    }

    #[test]
    fn test_random_boards() {
        let mut failed = false;

        for _ in 0..1000 {
            let bitboard: BitBoardBoard = random_board();
            let default_board: DefaultBoard = convert_board(&bitboard);
            let game = DefaultGame { board: default_board, next_turn: Black };

            let default_moves = game.valid_moves(Colour::Black);
            let bb_moves = bitboard.moves(Colour::Black);

            let default_moves_as_bitboard = default_moves.iter()
                .map(|mov| BitBoard::from((mov.row, mov.col)))
                .fold(BitBoard::new(), |b1, b2| b1 | b2);
            if default_moves_as_bitboard != bb_moves.1 {
                println!("Game =\n{:?}", game);
                println!("Default =\n{:?}", default_moves_as_bitboard);
                println!("BitBoard =\n{:?}", bb_moves.0);
                println!();
                failed = true;
            }
        }

        if failed {
            panic!("at least one board didn't match");
        }
    }
}
