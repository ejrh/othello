use othello::ai::minimax::evaluate_to_depth;
use othello::ai::evaluate_immediate;
use othello::game::{Colour, Game, Score};

#[test]
fn test_depth_0() {
    let game: Game = "●○○○".try_into().expect("ok");

    let score = evaluate_to_depth(&game, Colour::Black, 0);
    assert_eq!(2, score);

    let score = evaluate_to_depth(&game, Colour::White, 0);
    assert_eq!(-2, score);
}

#[test]
fn test_depth_1() {
    let mut game: Game = "\n\
    ·●○○○\n\
    ·○○\n\
    ·○".try_into().expect("ok");

    /* Estimate the value of a game assuming the opponent makes its best move, i.e. the worst
       move for us! */
    fn estimate_game(game: &Game) -> Score {
        let mut best_score = Score::MAX;
        let mut best_move = None;
        for mov in game.valid_moves(game.next_turn) {
            let game2 = game.apply(mov);
            let score = evaluate_immediate(&game2, Colour::Black);
            println!("{mov:?} yields score {score} with game\n{game2:?}");
            if score < best_score {
                best_score = score;
                best_move = Some(mov);
            }
        }
        println!("Best score for opponent is therefore {best_score} on {best_move:?}");
        best_score
    }

    /* Assume Black has just made a move and wants to evaluate the resulting game. */
    game.next_turn = Colour::White;
    let expected_score = estimate_game(&game);

    let score = evaluate_to_depth(&game, Colour::Black, 1);
    assert_eq!(expected_score, score);

    //TODO we can't test this, as evaluate_to_depth currently has some confusion about
    // whether to use the given player parameter or the next_player field of the game
    // let score = evaluate_to_depth(&game, Colour::White, 1);
    // assert_eq!(-expected_score, score);
}
