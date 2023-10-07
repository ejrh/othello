use crate::ai::{AI, evaluate_immediate, pick_best_move, Score};
use crate::game::{Colour, Game, Move};

pub struct MinimaxAI {
    pub max_depth: usize,
}

impl AI for MinimaxAI {
    fn choose_move(&self, game: &Game) -> Option<Move> {
        pick_best_move(game, |g, m| evaluate_to_depth(
            &g.apply(m),
            game.next_turn,
            self.max_depth))
    }

}

fn evaluate_to_depth(game: &Game, player: Colour, depth: usize) -> Score {
    if depth == 0 {
        evaluate_immediate(game, player)
    } else {
        /* Evaluate this position as if the opponent will make its best available move. */
        let opponent = player.opponent();
        let best_score = game.valid_moves()
            .map(|m| game.apply(m))
            .map(|g| -evaluate_to_depth(&g, opponent, depth - 1)).max();

        best_score.unwrap_or_else(|| evaluate_immediate(game, player))
    }
}