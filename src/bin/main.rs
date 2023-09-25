use rand::seq::SliceRandom;

use othello::game::Game;

fn main() {
    println!("Othello");

    let mut game = Game::new();

    println!("Game: {:?}", &game);

    loop {
        let moves = game.valid_moves();
        let mov = moves.choose(&mut rand::thread_rng());
        let Some(mov) = mov else {
            println!("No more moves!");
            break;
        };

        println!("Move: {:?}", mov);
        game = game.apply(mov);

        println!("Game:\n{:?}", &game);
    }
}
