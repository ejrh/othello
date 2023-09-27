use rand::seq::SliceRandom;

use othello::ai::{evaluate_immediate, Score};
use othello::game::{Colour, Game, Move};

fn simulate_one_game() -> Game {
    let mut game = Game::new();

    // println!("Game: {:?}", &game);

    loop {
        let mut moves = game.valid_moves();
        let mov = if game.next_turn == Colour::Black {
            fn evaluate_move(game: &Game, mov: &Move) -> Score {
                let game2 = game.apply(mov);
                -evaluate_immediate(&game2)
            }
            moves.sort_by_cached_key(|m| evaluate_move(&game, m));
            moves.get(0)
        } else {
            moves.choose(&mut rand::thread_rng())
        };

        let Some(mov) = mov else {
            // println!("No more moves!");
            break;
        };

        // println!("Move: {:?}", mov);
        game = game.apply(mov);
    }

    game
}

fn main() {
    println!("Othello");

    let mut total_score = 0;

    for _ in 0..1000 {
        let game = simulate_one_game();
        let score = evaluate_immediate(&game);
        // println!("Game: score={}\n{:?}", score, &game);
        total_score += score;
    }

    println!("Total score={}", total_score);
}
