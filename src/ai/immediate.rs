use crate::ai::{AI, evaluate_immediate, Score};
use crate::game::{Game, Move};

pub struct ImmediateAI {
}

impl AI for ImmediateAI {
    fn choose_move(&self, game: &Game) -> Option<Move> {

        fn evaluate_move(game: &Game, mov: Move) -> Score {
            let game2 = game.apply(mov);
            evaluate_immediate(&game2)
        }

        let mut moves = game.valid_moves();
        let mut best_move = moves.next();
        if best_move.is_some() {
            let mut best_score = evaluate_move(game, best_move.unwrap());
            for m in moves {
                let new_score =  evaluate_move(game, m);
                if new_score > best_score {
                    best_score = new_score;
                    best_move = Some(m);
                }
            }
        }
        best_move
    }
}
