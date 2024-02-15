use crate::{AI, evaluate_immediate, pick_best_move, Score};
use othello_game::{Board, Colour, Game, Move};

#[derive(Clone)]
pub struct AlphaBetaAI {
    pub max_depth: usize,
}

impl AI for AlphaBetaAI {
    fn choose_move<B: Board>(&self, game: &Game<B>) -> Option<Move> {
        pick_best_move(game, |g, m| evaluate_to_depth(
            &g.apply(m),
            game.next_turn,
            -1_000_000,
            1_000_000,
            self.max_depth))
    }
}

fn evaluate_to_depth<B: Board>(game: &Game<B>, player: Colour, mut alpha: Score, beta: Score, depth: usize) -> Score {
    if depth == 0 {
        evaluate_immediate(game, player)
    } else {
        /* Evaluate this position as if the opponent will make its best available move. */
        let opponent = player.opponent();
        for mov in game.valid_moves(player) {
            let g = game.apply(mov);
            let score = -evaluate_to_depth(&g, opponent, -beta, -alpha, depth - 1);
            if score >= beta { return beta }
            if score > alpha { alpha = score }
        }

        alpha
    }
}
