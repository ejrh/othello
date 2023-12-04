use rand::prelude::SliceRandom;

use othello::game::{DefaultGame, Move};

fn random_game() {
    let mut num_turns: usize = 0;
    let mut total_moves = 0;

    let mut game = DefaultGame::new();

    loop {
        /* Print current game state */
        print!("Turn {num_turns}, game state is:\n{game:?}");
        let (black_score, white_score) = game.scores();
        println!("Current score is {black_score} for black, {white_score} for white");
        let player = game.next_turn;
        let moves: Vec<Move> = game.valid_moves(player).into_iter().collect();
        let num_moves = moves.len();
        println!("{player:?} to move, {num_moves} moves available");

        num_turns += 1;
        total_moves += num_moves;

        if moves.is_empty() { break }

        /* Make a random move */
        let mov = moves.choose(&mut rand::thread_rng()).expect("at least one move");
        game = game.apply(*mov);
    }
    let branching_factor = total_moves as f64/num_turns as f64;
    println!("Average branching factor was {branching_factor:0.2} ({total_moves} moves / {num_turns} turns)");
}

fn main() {
    random_game();
}
