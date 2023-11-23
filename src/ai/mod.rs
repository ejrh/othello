mod immediate;
pub mod minimax;
mod random;

use crate::game::{Colour, Board, Game, Move, Score};

pub use immediate::ImmediateAI;
pub use minimax::MinimaxAI;
pub use random::RandomAI;

/**
 * Evaluate this immediate game position, returning a `Score`.  A higher score is considered
 * better.  Evaluation is done from the point of view of the given player, using the "negamax" approach.
 *
 * Currently, the evaluation is simply the count of friendly pieces minus the count of enemy pieces.
 */
pub fn evaluate_immediate<B: Board>(game: &Game<B>, player: Colour) -> Score {
    let (black_count, white_count) = game.board.scores();
    let score = black_count - white_count;
    score * player.sign()
}

/**
 * Pick the best move in the game, for the current player, using the given evaluation function.
 * This will pick the move with the highest score (as calculated by the evaluation function on the
 * game position resulting from that move).   Higher scores are better, as in the "negamax"
 * approach.
 */
pub fn pick_best_move<B: Board, F>(game: &Game<B>, evaluate_move: F) -> Option<Move>
where F: Fn(&Game<B>, Move) -> Score {
    // TODO - don't bother to call the evaluate_move function if there is only one move available
    game.valid_moves(game.next_turn).into_iter().max_by_key(|m| evaluate_move(game, *m))
}

pub trait AI: Clone + Send {
    fn choose_move<B: Board>(&self, game: &Game<B>) -> Option<Move>;
}
