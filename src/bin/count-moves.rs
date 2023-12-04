use rand::prelude::SliceRandom;

use othello::game::DefaultGame;

fn run_one_game() -> (usize, usize, usize, Option<DefaultGame>) {
    let mut turns: usize = 0;
    let mut total_moves = 0;
    let mut max_moves = 0;
    let mut max_moves_game = None;
    let mut game = DefaultGame::new();

    loop {
        let moves: Vec<_> = game.valid_moves(game.next_turn).into_iter().collect();
        if moves.is_empty() { break }

        let num = moves.len();
        turns += 1;
        total_moves += num;
        if num > max_moves {
            max_moves = num;
            max_moves_game = Some(game.clone());
        }

        let mov = moves.choose(&mut rand::thread_rng()).expect("at least one move");
        game = game.apply(*mov);
    }

    (turns, total_moves, max_moves, max_moves_game)
}

fn main() {
    let mut total_max_moves = 0;
    let mut total_max_moves_game;
    for game_no in 0..1000000 {
        let (_turns, _total_moves, max_moves, max_moves_game) = run_one_game();
        //println!("turns {} moves {} max {}", turns, total_moves, max_moves);
        if max_moves >= total_max_moves {
            total_max_moves = max_moves;
            total_max_moves_game = max_moves_game;
            println!("{}: {}", game_no, total_max_moves);
            println!("{:?}", total_max_moves_game.unwrap());
        }
    }
}
