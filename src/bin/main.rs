use std::sync::atomic::{AtomicIsize, AtomicUsize, Ordering};
use std::thread;
use std::time::SystemTime;

use othello::ai::{AI, evaluate_immediate, MinimaxAI, RandomAI};
use othello::game::{Colour, Game};

fn simulate_one_game(black_ai: impl AI, white_ai: impl AI) -> Game {
    let mut game = Game::new();
    // println!("Game: {:?}", &game);

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

    thread::scope(|s| {
        for _ in 0..num_threads {
            let black_ai = black_ai.clone();
            let white_ai = white_ai.clone();
            s.spawn(|| {
                let games_per_thread = num_games.div_ceil(num_threads);
                let (black_ai, white_ai) = (black_ai, white_ai);
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

    let black_ai = MinimaxAI { max_depth: 3 };
    let white_ai = RandomAI { };

    let num_games = 10000;
    let num_threads = thread::available_parallelism()
        .map_or(1, |x| x.get());
    let t0 = SystemTime::now();
    let (total_score, games_run) = simulate_many_games_in_parallel(&black_ai, &white_ai, num_games, num_threads);
    println!("Simulating {} games on {} threads took {:?}",
             games_run, num_threads, t0.elapsed().expect("no time travel"));

    println!("Average score={:2.2}", total_score as f64 / games_run as f64);
}
