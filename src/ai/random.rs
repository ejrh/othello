use rand::seq::SliceRandom;

use crate::ai::AI;
use crate::game::{Board, Game, Move};

#[derive(Clone)]
pub struct RandomAI {
}

impl AI for RandomAI {
    fn choose_move<B: Board>(&self, game: &Game<B>) -> Option<Move> {
        let moves: Vec<Move> = game.valid_moves(game.next_turn).into_iter().collect();
        moves.choose(&mut rand::thread_rng()).copied()
    }
}
