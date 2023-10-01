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

pub fn pick_best_move<F>(game: &Game, evaluate_move: F) -> Option<Move>
where F: Fn(&Game, Move) -> Score {
    let mut moves = game.valid_moves();
    let mut best_move = moves.next();
    if best_move.is_some() {
        let mut best_score = evaluate_move(game, best_move.unwrap());
        for m in moves {
            let new_score =  evaluate_move(game, m);
            if new_score > best_score {
                best_score = new_score;
                best_move = Some(m);
            }
        }
    }
    best_move
}

pub trait AI {
    fn choose_move(&self, game: &Game) -> Option<Move>;
}
