use std::sync::atomic::Ordering;
use crate::{AI, AIInfo, evaluate_immediate, pick_best_move, Score};
use othello_game::{Board, Colour, Game, Move};

#[derive(Clone)]
pub struct MinimaxAI {
    pub max_depth: usize,
    info: AIInfo,
}

impl MinimaxAI {
    pub fn new(max_depth: usize) -> Self {
        let info = AIInfo::default();
        MinimaxAI { max_depth, info }
    }
}

impl AI for MinimaxAI {
    fn choose_move<B: Board>(&self, game: &Game<B>) -> Option<Move> {
        pick_best_move(game, |g, m| evaluate_to_depth(
            &g.apply(m),
            game.next_turn,
            self.max_depth,
        &self.info))
    }

    fn info(&self) -> Option<&AIInfo> {
        Some(&self.info)
    }
}

pub fn evaluate_to_depth<B: Board>(game: &Game<B>, player: Colour, depth: usize, info: &AIInfo) -> Score {
    info.add_node();

    if depth == 0 {
        evaluate_immediate(game, player)
    } else {
        /* Evaluate this position as if the opponent will make its best available move. */
        let opponent = player.opponent();
        let best_score = game.valid_moves(opponent)
            .into_iter()
            .map(|m| game.apply(m))
            .map(|g| -evaluate_to_depth(&g, opponent, depth - 1, &info)).min();

        best_score.unwrap_or_else(|| evaluate_immediate(game, player))
    }
}
