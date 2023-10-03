use std::cmp::max;

use crate::ai::{AI, evaluate_immediate, pick_best_move, Score};
use crate::game::{Colour, Game, Move};

pub struct MinimaxAI {
    pub max_depth: usize,
}

impl AI for MinimaxAI {
    fn choose_move(&self, game: &Game) -> Option<Move> {
        pick_best_move(&game, |g, m| evaluate_to_depth(
            &g.apply(m),
            game.next_turn,
            self.max_depth))
    }

}

fn evaluate_to_depth(game: &Game, player: Colour, depth: usize) -> Score {
    if depth == 0 {
        evaluate_immediate(&game, player)
    } else {
        /* Evaluate this position as if the opponent will make its best available move. */
        let mut best_score = None;
        let opponent = player.opponent();
        for mov in game.valid_moves() {
            let game2 = game.apply(mov);
            let score = -evaluate_to_depth(&game2, opponent, depth - 1);
            if best_score.is_none() {
                best_score = Some(score);
            } else {
                best_score = Some(max(best_score.unwrap(), score));
            }
        }

        best_score.unwrap_or_else(|| evaluate_immediate(&game, player))
    }
}
