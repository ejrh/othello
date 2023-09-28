use rand::seq::SliceRandom;

use crate::ai::AI;
use crate::game::{Game, Move};

pub struct RandomAI {
}

impl AI for RandomAI {
    fn choose_move(&self, game: &Game) -> Option<Move> {
        let moves: Vec<Move> = game.valid_moves().collect();
        moves.choose(&mut rand::thread_rng()).copied()
    }
}
