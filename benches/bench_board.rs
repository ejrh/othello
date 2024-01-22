use criterion::{black_box, criterion_group, criterion_main, Criterion};

use othello::game::bitboardgame::BitBoardBoard;
use othello::game::{Board, Colour, convert_board, random_board};
use othello::game::default::DefaultBoard;

pub fn board_benchmark(c: &mut Criterion) {
    const NUM_BOARDS: usize = 100;

    let boards: Vec<BitBoardBoard> = (0..NUM_BOARDS).map(|_| random_board()).collect();

    c.bench_function("bitboard board", |b| {
        b.iter(|| {
            for bb in &boards {
                black_box(bb.moves(Colour::Black));
                black_box(bb.moves(Colour::White));
            }
        });
    });

    let boards: Vec<DefaultBoard> = boards.iter().map(convert_board).collect();

    c.bench_function("default board", |b| {
        b.iter(|| {
            for bb in &boards {
                black_box(bb.moves(Colour::Black));
                black_box(bb.moves(Colour::White));
            }
        });
    });
}

criterion_group!(benches, board_benchmark);
criterion_main!(benches);
