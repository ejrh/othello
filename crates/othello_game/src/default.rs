use crate::{Board, BOARD_SIZE, Colour, Move, Pos, Score};
use crate::direction::{DIRECTIONS, IterateFrom};

#[derive(Clone, Default, PartialEq)]
pub(crate) struct Square {
    pub(crate) piece: Option<Colour>
}

#[derive(Clone, Default, PartialEq)]
pub struct DefaultBoard {
    squares: [[Square; BOARD_SIZE as usize]; BOARD_SIZE as usize]
}

impl DefaultBoard {
    fn flip(&mut self, player: Colour, row: Pos, col: Pos, dy: Pos, dx: Pos) {
        let mut iter = (dy, dx).iterate_from(row, col);

        loop {
            let Some((r, c)) = iter.next() else { return; };
            let Some(colour) = self.squares[r as usize][c as usize].piece else { return; };

            if colour == player { break; }
        }

        let mut iter = (dy, dx).iterate_from(row, col);
        loop {
            let Some((r, c)) = iter.next() else { return; };
            let Some(colour) = self.squares[r as usize][c as usize].piece else { return; };

            if colour == player { break; }

            self.squares[r as usize][c as usize].piece = Some(player);
        }
    }

    #[inline(always)]
    fn count_in_dir(&self, player: Colour, row: Pos, col: Pos, dy: Pos, dx: Pos) -> usize {
        let mut count = 0;
        let mut iter = (dy, dx).iterate_from(row, col);

        loop {
            let Some((r, c)) = iter.next() else { return 0; };
            let Some(colour) = self.squares[r as usize][c as usize].piece else { return 0; };

            if colour == player { break; }
            count += 1;
        }

        count
    }
}

impl Board for DefaultBoard {
    type MoveSet = Vec<Move>;

    #[inline(always)]
    fn is_valid_move(&self, mov: Move) -> bool {
        //if mov.player != self.next_turn { return false; }
        if self.squares[mov.row as usize][mov.col as usize].piece.is_some() { return false; }

        //TODO - this is one of the hottest loops in the whole program, so anything that might
        // speed it up (*cough* bitboards) is worth looking into
        DIRECTIONS.iter()
            .any(|(dy, dx)| self.count_in_dir(mov.player, mov.row, mov.col, *dy, *dx) > 0)
    }

    fn moves(&self, for_player: Colour) -> Self::MoveSet {
        (0..8).flat_map(|i| (0..8)
            .map(move |j| Move { player: for_player, row: i, col: j}))
            .filter(|mov| self.is_valid_move(*mov)).collect()
    }

    fn apply(&self, mov: Move) -> Self {
        assert!(self.is_valid_move(mov));
        let mut newboard = (*self).clone();

        DIRECTIONS.iter()
            .for_each(|(dy, dx)| newboard.flip(mov.player, mov.row, mov.col, *dy, *dx));

        newboard.squares[mov.row as usize][mov.col as usize].piece = Some(mov.player);

        newboard
    }

    fn get(&self, row: Pos, col: Pos) -> Option<Colour> {
        self.squares[row as usize][col as usize].piece
    }

    fn set(&mut self, row: Pos, col: Pos, value: Option<Colour>) {
        self.squares[row as usize][col as usize].piece = value;
    }

    fn scores(&self) -> (Score, Score) {
        let mut black_count = 0;
        let mut white_count = 0;
        self.squares.iter()
            .flat_map(|r| r.iter())
            .flat_map(|sq| sq.piece)
            .for_each(|c| if c == Colour::Black { black_count += 1; } else { white_count += 1; });
        (black_count, white_count)
    }
}
