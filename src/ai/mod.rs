mod immediate;
mod random;

use crate::game::{Colour, Game, Move};

pub use immediate::ImmediateAI;
pub use random::RandomAI;

pub type Score = i32;

/**
 * Evaluate this immediate game position, returning a `Score`.  A higher score is considered
 * better.  Evaluation is done from the point of view of the given player, using the "negamax" approach.
 *
 * Currently, the evaluation is simply the count of friendly pieces minus the count of enemy pieces.
 */
pub fn evaluate_immediate(game: &Game, player: Colour) -> Score {
    let mut score = 0;

    for row in &game.board {
        for square in row {
            let Some(colour) = square.piece
            else { continue; };

            let val = if colour == player { 1 } else { -1 };
            score += val;
        }
    }

    score
}

/**
 * Pick the best move in the game, for the current player, using the given evaluation function.
 * This will pick the move with the highest score (as calculated by the evaluation function on the
 * game position resulting from that move).   Higher scores are better, as in the "negamax"
 * approach.
 */
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
