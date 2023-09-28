mod immediate;
mod random;

use crate::game::{Colour, Game, Move};

pub use immediate::ImmediateAI;
pub use random::RandomAI;

pub type Score = i32;

pub fn evaluate_immediate(game: &Game) -> Score {
    let mut score = 0;

    for row in &game.board {
        for square in row {
            let Some(colour) = square.piece
            else { continue; };

            let val = match colour {
                Colour::Black => 1,
                Colour::White => -1,
            };
            score += val;
        }
    }

    score
}

pub trait AI {
    fn choose_move(&self, game: &Game) -> Option<Move>;
}
