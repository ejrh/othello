use crate::{AI, evaluate_immediate, pick_best_move, Score};
use othello_game::{convert, Board, Game, GameRepr, Move};
use othello_game::bitboardgame::BitBoardBoard;

#[derive(Clone)]
pub struct ImmediateAI {
}

impl AI for ImmediateAI {
    fn choose_move(&self, game: &dyn Game) -> Option<Move> {
        fn evaluate_move<B: Board>(game: &GameRepr<B>, mov: Move) -> Score {
            let game2 = game.apply(mov);
            evaluate_immediate(&game2, game.next_turn())
        }

        let game: GameRepr<BitBoardBoard> = convert(game);

        pick_best_move(&game, evaluate_move)
    }
}
