use std::cmp::min;
use std::sync::atomic::{AtomicIsize, AtomicUsize, Ordering};
use std::thread;
use std::time::SystemTime;

use othello_ai::{AI, evaluate_immediate, AlphaBetaAI, RandomAI};
use othello_game::{Colour, DefaultGame, Game};

fn simulate_one_game(black_ai: impl AI, white_ai: impl AI) -> Game {
    let mut game = DefaultGame::new();
    // println!("Game: {:?}", &othello_game);

    loop {
        let mov = if game.next_turn == Colour::Black { black_ai.choose_move(&game) }
            else { white_ai.choose_move(&game) };

        let Some(mov) = mov else {
            // println!("No more moves!");
            break;
        };

        // println!("Move: {:?}", mov);
        game = game.apply(mov);
    }

    game
}

fn simulate_many_games(black_ai: &impl AI, white_ai: &impl AI, num_games: usize) -> isize {
    let mut total_score = 0;
    for _ in 0..num_games {
        let game = simulate_one_game(black_ai.clone(), white_ai.clone());
        let score = evaluate_immediate(&game, Colour::Black);
        total_score += score as isize;
    }
    total_score
}

fn simulate_many_games_in_parallel(black_ai: &impl AI, white_ai: &impl AI, num_games: usize, num_threads: usize) -> (isize, usize) {
    let total_score = AtomicIsize::new(0);
    let games_run: AtomicUsize = AtomicUsize::new(0);

    fn make_chunks(mut total: usize, num_chunks: usize) -> Vec<usize> {
        let mut chunks = Vec::new();
        let mut chunk_size = total / num_chunks;
        if chunk_size * num_chunks < total { chunk_size += 1 }
        while total >= 1 {
            let size = min(chunk_size, total);
            total -= size;
            chunks.push(size);
        }
        chunks
    }

    thread::scope(|s| {
        for games_per_thread in make_chunks(num_games, num_threads) {
            // Make copies of the shared objects to move into this thread's closure
            let black_ai = black_ai.clone();
            let white_ai = white_ai.clone();
            let total_score = &total_score;
            let games_run = &games_run;

            s.spawn(move || {
                let thread_score = simulate_many_games(&black_ai, &white_ai, games_per_thread);
                total_score.fetch_add(thread_score, Ordering::Relaxed);
                games_run.fetch_add(games_per_thread, Ordering::Relaxed);
            });
        }
    });
    let total_score = total_score.into_inner();
    let games_run = games_run.into_inner();

    (total_score, games_run)
}

fn main() {
    println!("Othello");

    let black_ai = AlphaBetaAI { max_depth: 3 };
    let white_ai = RandomAI { };

    let num_games = 1000;
    let num_threads = thread::available_parallelism()
        .map_or(1, |x| x.get());
    let t0 = SystemTime::now();
    let (total_score, games_run) = simulate_many_games_in_parallel(&black_ai, &white_ai, num_games, num_threads);
    println!("Simulating {} games on {} threads took {:?}",
             games_run, num_threads, t0.elapsed().expect("no time travel"));

    println!("Average score={:2.2}", total_score as f64 / games_run as f64);
}
