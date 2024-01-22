use criterion::{black_box, criterion_group, criterion_main, Criterion};
use othello::ai::{AI, MinimaxAI};

use othello::game::bitboardgame::BitBoardBoard;
use othello::game::{Board, Colour, convert_board, DefaultGame, random_board};
use othello::game::default::DefaultBoard;

pub fn minimax_benchmark(c: &mut Criterion) {
    const NUM_BOARDS: usize = 100;

    let boards: Vec<DefaultBoard> = (0..NUM_BOARDS).map(|_| random_board()).collect();

    let ai = MinimaxAI { max_depth: 3 };
    
    c.bench_function("minimax", |b| {
        b.iter(|| {
            for bb in &boards {
                let game = DefaultGame { board: bb.clone(), next_turn: Colour::Black };
                let mov = ai.choose_move(&game);
                black_box(mov);
            }
        });
    });
}

criterion_group!(benches, minimax_benchmark);
criterion_main!(benches);
