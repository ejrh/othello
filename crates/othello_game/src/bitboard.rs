use std::fmt::{Debug, Display, Formatter, Write};
use std::ops::{BitAnd, BitOr, BitAndAssign, BitOrAssign, Not};

#[derive(Clone, Copy)]
pub(crate) enum ShiftDir {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl ShiftDir {
    pub(crate) fn shift(&self) -> i8 {
        match self {
            ShiftDir::Up => -8,
            ShiftDir::Down => 8,
            ShiftDir::Left => -1,
            ShiftDir::Right => 1,
            ShiftDir::UpLeft => -9,
            ShiftDir::UpRight => -7,
            ShiftDir::DownLeft => 7,
            ShiftDir::DownRight => 9,
        }
    }

    pub(crate) fn reverse(&self) -> ShiftDir {
        match self {
            ShiftDir::Up => ShiftDir::Down,
            ShiftDir::Down => ShiftDir::Up,
            ShiftDir::Left => ShiftDir::Right,
            ShiftDir::Right => ShiftDir::Left,
            ShiftDir::UpLeft => ShiftDir::DownRight,
            ShiftDir::UpRight => ShiftDir::DownLeft,
            ShiftDir::DownLeft => ShiftDir::UpRight,
            ShiftDir::DownRight => ShiftDir::UpLeft,
        }
    }
}

pub(crate) const SHIFT_DIRS: &[ShiftDir] = &[
    ShiftDir::Up, ShiftDir::Down, ShiftDir::Left, ShiftDir::Right,
    ShiftDir::UpLeft, ShiftDir::UpRight, ShiftDir::DownLeft, ShiftDir::DownRight,
];

#[derive(Clone, Copy, Default, PartialEq)]
pub struct BitBoard(u64);

impl BitBoard {
    pub(crate) fn new() -> BitBoard {
        BitBoard(0)
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.0 == 0
    }

    pub(crate) fn bit(&self, row: i8, col: i8) -> bool {
        ((self.0 >> (row * 8 + col)) & 1) != 0
    }

    pub(crate) fn count(&self) -> u32 {
        self.0.count_ones()
    }

    pub(crate) fn next_bit(&self) -> BitBoard {
        if self.0 == 0 {
            return *self;
        }
        let new_val = self.0 & (self.0 - 1);
        let lsb = self.0 - new_val;
        BitBoard(lsb)
    }

    pub(crate) fn pop_next_bit(&mut self) -> BitBoard {
        if self.0 == 0 {
            return *self;
        }
        let new_val = self.0 & (self.0 - 1);
        let lsb = self.0 - new_val;
        self.0 = new_val;
        BitBoard(lsb)
    }

    pub(crate) fn to_bit_pos(self) -> (i8, i8) {
        let bit_num = self.0.trailing_zeros() as i8;
        (bit_num >> 3, bit_num & 7)
    }

    #[inline(always)]
    pub(crate) fn shift(&self, shift: i8) -> BitBoard {
        let x = self.0;
        let mut x = if shift < 0 {
            x >> (-shift)
        } else {
            x << shift
        };
        match shift {
            -1 | 7 | -9 => x &= 0x7F7F7F7F7F7F7F7F,
            1 | -7 | 9 => x &= 0xFEFEFEFEFEFEFEFE,
            _ => ()
        }
        BitBoard(x)
    }
}

impl From<(i8, i8)> for BitBoard {
    fn from(value: (i8, i8)) -> Self {
        let mut bb = BitBoard::new();

        let (row, col) = value;
        bb.0 |= 1 << (row * 8 + col);

        bb
    }
}

impl From<&[(i8, i8)]> for BitBoard {
    fn from(value: &[(i8, i8)]) -> Self {
        let mut bb = BitBoard::new();

        for (row, col) in value {
            bb.0 |= 1 << (row * 8 + col);
        }

        bb
    }
}

impl<const N: usize> From<&[(i8, i8); N]> for BitBoard {
    fn from(value: &[(i8, i8); N]) -> Self {
        Self::from(&value[..])
    }
}

impl From<&str> for BitBoard {
    fn from(value: &str) -> Self {
        let mut bb = BitBoard::new();
        for (i, line) in value.split_terminator('\n').enumerate() {
            for (j, ch) in line.chars().enumerate() {
                if ch == 'X' {
                    bb |= BitBoard::from((i as i8, j as i8));
                }
            }
        }
        bb
    }
}

impl Debug for BitBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut remaining_bits = self.0;
        for i in 0..8 {
            for j in 0..8 {
                let ch = if self.bit(i, j) { 'X' } else { '·' };
                f.write_char(ch)?;
            }

            /* If no bits left, don't bother emitting any more lines */
            remaining_bits >>= 8;
            if remaining_bits == 0 {
                break
            }

            if i != 7 {
                f.write_char('\n')?;
            }
        }
        Ok(())
    }
}

impl Display for BitBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in 0..8 {
            for j in 0..8 {
                let ch = if self.bit(i, j) { 'X' } else { '·' };
                f.write_char(ch)?;
            }
            if i != 7 {
                f.write_char('\n')?;
            }
        }
        Ok(())
    }
}

impl BitAnd for BitBoard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitOr for BitBoard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitAndAssign for BitBoard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl BitOrAssign for BitBoard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl Not for BitBoard {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

/**
 * A regular dumb7fill, adapted from https://www.chessprogramming.org/Dumb7Fill.
 */
#[inline(always)]
pub(crate) fn dumb7fill(mut gen: BitBoard, pro: BitBoard, shift: i8) -> BitBoard {
    let mut flood = gen;
    for _ in 1..7 {
        gen = gen.shift(shift) & pro;
        flood |= gen;
    }
    flood
}

/**
 * An occluded dumb7fill, adapted from https://www.chessprogramming.org/Dumb7Fill.
 */
#[inline(always)]
pub(crate) fn dumb7fill_occluded(mut gen: BitBoard, pro: BitBoard, shift: i8) -> BitBoard {
    let mut flood = BitBoard::new();
    for _ in 1..7 {
        gen = gen.shift(shift) & pro;
        flood |= gen;
    }
    flood
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_next_bit() {
        let bb = BitBoard::from(&[(0, 0), (0, 6)]);
        let nb = bb.next_bit();
        assert_eq!(BitBoard::from(&[(0, 0)]), nb);
    }

    #[test]
    fn test_from() {
        let bb = BitBoard::from((4, 2));
        assert_eq!(1 << (4 * 8 + 2), bb.0);
        assert_eq!(true, bb.bit(4, 2));
        assert_eq!(1, bb.count());

        let bb = BitBoard::from("X·····X·");
        assert_eq!(BitBoard::from(&[(0, 0), (0, 6)]), bb);
    }

    #[test]
    fn test_tostr() {
        let bb = BitBoard::from((4, 2));
        let s: String = bb.to_string();
        assert_eq!("········\n········\n········\n········\n··X·····\n········\n········\n········", &s);
    }

    #[test]
    fn test_shift() {
        let bb = BitBoard::from(&[(4, 0), (3, 3)]);

        assert_eq!(BitBoard::from(&[(3, 0), (2, 3)]), bb.shift(ShiftDir::Up.shift()));
        assert_eq!(BitBoard::from(&[(5, 0), (4, 3)]), bb.shift(ShiftDir::Down.shift()));

        assert_eq!(BitBoard::from((3, 2)), bb.shift(ShiftDir::Left.shift()));
        assert_eq!(BitBoard::from(&[(4, 1), (3, 4)]), bb.shift(ShiftDir::Right.shift()));
    }

    #[test]
    fn test_dumb7fill() {
        let gen = BitBoard::from("X··X···X");
        let pro = BitBoard::from("·XXX··X·");

        let filled = dumb7fill_occluded(gen, pro, -1);
        assert_eq!(BitBoard::from("·XX···X·"), filled);
    }
}
