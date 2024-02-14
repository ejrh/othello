use crate::{out_of_range, Pos};

pub(crate) const DIRECTIONS: &[(Pos, Pos)] = &[
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1)
];

pub(crate) struct DirectionIterator {
    dx: Pos,
    dy: Pos,
    row: Pos,
    col: Pos
}

impl Iterator for DirectionIterator {
    type Item = (Pos, Pos);

    fn next(&mut self) -> Option<Self::Item> {
        self.row += self.dy;
        self.col += self.dx;
        if out_of_range(self.row, self.col) { return None; }
        Some((self.row, self.col))
    }
}

pub(crate) trait IterateFrom {
    fn iterate_from(&self, row: Pos, col: Pos) -> DirectionIterator;
}

impl IterateFrom for (Pos, Pos) {

    fn iterate_from(&self, row: Pos, col: Pos) -> DirectionIterator {
        DirectionIterator { dx: self.0, dy: self.1, row, col }
    }
}
