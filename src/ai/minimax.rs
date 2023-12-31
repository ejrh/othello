use crate::ai::{AI, evaluate_immediate, pick_best_move, Score};
use crate::game::{Board, Colour, Game, Move};

#[derive(Clone)]
pub struct MinimaxAI {
    pub max_depth: usize,
}

impl AI for MinimaxAI {
    fn choose_move<B: Board>(&self, game: &Game<B>) -> Option<Move> {
        pick_best_move(game, |g, m| evaluate_to_depth(
            &g.apply(m),
            game.next_turn,
            self.max_depth))
    }

}

pub fn evaluate_to_depth<B: Board>(game: &Game<B>, player: Colour, depth: usize) -> Score {
    if depth == 0 {
        evaluate_immediate(game, player)
    } else {
        /* Evaluate this position as if the opponent will make its best available move. */
        let opponent = player.opponent();
        let best_score = game.valid_moves(opponent)
            .into_iter()
            .map(|m| game.apply(m))
            .map(|g| -evaluate_to_depth(&g, opponent, depth - 1)).min();

        best_score.unwrap_or_else(|| evaluate_immediate(game, player))
    }
}
