use std::num::NonZeroUsize;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::SystemTime;
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

    let total_score = AtomicUsize::new(0);
    let games_run: AtomicUsize = AtomicUsize::new(0);
    const NUM_GAMES: usize = 100000;
    let t0 = SystemTime::now();
    let num_threads = thread::available_parallelism()
        .unwrap_or(NonZeroUsize::new(1).expect("1 != 0"));
    let num_threads = num_threads.get();
    thread::scope(|s| {
        for _ in 0..num_threads {
            s.spawn(|| {
                let mut thread_total_score: usize = 0;
                for _ in 0..(NUM_GAMES / num_threads) {
                    let game = simulate_one_game();
                    let score = evaluate_immediate(&game);
                    // println!("Game: score={}\n{:?}", score, &game);
                    thread_total_score += score as usize;
                }
                total_score.fetch_add(thread_total_score, Ordering::Relaxed);
                games_run.fetch_add(NUM_GAMES / num_threads, Ordering::Relaxed);
            });
        }
    });
    let total_score = total_score.into_inner();
    let games_run = games_run.into_inner();
    println!("Simulating {} games on {} threads took {:?}",
             games_run, num_threads, t0.elapsed().expect("no time travel"));

    println!("Average score={:2.2}", total_score as f64 / games_run as f64);
}
