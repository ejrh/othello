mod alphabeta;
mod immediate;
pub mod minimax;
mod random;

use std::cell::Cell;
use othello_game::{Board, Colour, Game, GameRepr, Move, Score};

pub use alphabeta::AlphaBetaAI;
pub use immediate::ImmediateAI;
pub use minimax::MinimaxAI;
pub use random::RandomAI;

/**
 * Evaluate this immediate othello_game position, returning a `Score`.  A higher score is considered
 * better.  Evaluation is done from the point of view of the given player, using the "negamax" approach.
 *
 * Currently, the evaluation is simply the count of friendly pieces minus the count of enemy pieces.
 */
pub fn evaluate_immediate(game: &impl Game, player: Colour) -> Score {
    let (black_count, white_count) = game.scores();
    let score = black_count - white_count;
    score * player.sign()
}

/**
 * Pick the best move in the othello_game, for the current player, using the given evaluation function.
 * This will pick the move with the highest score (as calculated by the evaluation function on the
 * othello_game position resulting from that move).   Higher scores are better, as in the "negamax"
 * approach.
 */
pub fn pick_best_move<F, B: Board>(game: &GameRepr<B>, evaluate_move: F) -> Option<Move>
where F: Fn(&GameRepr<B>, Move) -> Score {
    // TODO - don't bother to call the evaluate_move function if there is only one move available
    game.valid_moves(game.next_turn()).into_iter().max_by_key(|m| evaluate_move(game, *m))
}

pub trait AI: Clone + Send {
    fn choose_move(&self, game: &dyn Game) -> Option<Move>;
    fn info(&self) -> Option<AIInfo> { None }
}

#[derive(Clone, Default)]
pub struct AIInfo {
    pub total_nodes_searched: Cell<usize>,
    pub last_nodes_searched: Cell<usize>,
    pub last_num_choices: Cell<usize>,
}

unsafe impl Send for AIInfo {}
unsafe impl Sync for AIInfo {}

impl AIInfo {
    fn add_node(&self) {
        self.last_nodes_searched.update(|x| x + 1);
    }

    fn begin_search(&self, num_choices: usize) {
        self.last_num_choices.set(num_choices);
        self.last_nodes_searched.set(0);
    }

    fn finish_search(&self) {
        self.total_nodes_searched.update(|x| x + self.last_nodes_searched.get());
    }
}
