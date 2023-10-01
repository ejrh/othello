use std::fmt::Write;

use othello::game::{Colour, Game, Move};

#[test]
fn test_initial_layout() {
    let game = Game::new();

    assert_eq!(None, game.get_piece(0, 0));
    assert_eq!(Some(Colour::Black), game.get_piece(3, 3));
    assert_eq!(Some(Colour::White), game.get_piece(3, 4));
    assert_eq!(Some(Colour::White), game.get_piece(4, 3));
    assert_eq!(Some(Colour::Black), game.get_piece(4, 4));

    assert_eq!(Colour::Black, game.next_turn);
}

#[test]
fn test_debug() {
    let game = Game::new();

    let mut str = String::new();
    write!(&mut str, "{:?}", game).unwrap();

    assert_eq!("········\n········\n········\n\
        ···○●···\n\
        ···●○···\n\
        ········\n········\n········\n", str);
}

#[test]
fn test_initial_moves() {
    let game = Game::new();

    let mut moves = game.valid_moves();

    assert_eq!(Some(Move { player: Colour::Black, row: 2, col: 4 }), moves.next());
    assert_eq!(Some(Move { player: Colour::Black, row: 3, col: 5 }), moves.next());
    assert_eq!(Some(Move { player: Colour::Black, row: 4, col: 2 }), moves.next());
    assert_eq!(Some(Move { player: Colour::Black, row: 5, col: 3 }), moves.next());
    assert_eq!(None, moves.next());
}

#[test]
fn test_apply_move() {
    let game = Game::new();

    let mov = Move {
        player: Colour::Black,
        row: 2,
        col: 4,
    };

    let game2 = game.apply(mov);
    assert_eq!(Colour::White, game2.next_turn);

    let mut str = String::new();
    write!(&mut str, "{:?}", game2).unwrap();

    assert_eq!("········\n········\n\
        ····○···\n\
        ···○○···\n\
        ···●○···\n\
        ········\n········\n········\n", str);
}
