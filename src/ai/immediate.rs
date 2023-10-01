use crate::ai::{AI, evaluate_immediate, pick_best_move, Score};
use crate::game::{Game, Move};

pub struct ImmediateAI {
}

impl AI for ImmediateAI {
    fn choose_move(&self, game: &Game) -> Option<Move> {
        fn evaluate_move(game: &Game, mov: Move) -> Score {
            let game2 = game.apply(mov);
            evaluate_immediate(&game2)
        }

        pick_best_move(game, evaluate_move)
    }
}
