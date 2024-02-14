use crate::{AI, evaluate_immediate, pick_best_move, Score};
use othello_game::{Board, Game, Move};

#[derive(Clone)]
pub struct ImmediateAI {
}

impl AI for ImmediateAI {
    fn choose_move<B: Board>(&self, game: &Game<B>) -> Option<Move> {
        fn evaluate_move<B: Board>(game: &Game<B>, mov: Move) -> Score {
            let game2 = game.apply(mov);
            evaluate_immediate(&game2, game.next_turn)
        }

        pick_best_move(game, evaluate_move)
    }
}
