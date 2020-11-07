pub mod create;
pub mod rules;
pub mod solve;

use std::convert::TryInto;
use std::fmt;
use std::num::NonZeroU8;

#[derive(Clone, PartialEq)]
pub struct Board {
    // Rows are read from left to right and then top to bottom.
    pub(crate) cells: [Cell; 81],
}

impl Board {
    pub fn new(s: &str) -> Board {
        let mut cells = [Cell::Unfilled; 81];
        for (dst, src) in cells.iter_mut().zip(s.chars().map(|c| {
            let digit = c.to_digit(10).unwrap();
            let digit: u8 = digit.try_into().unwrap();
            if digit == 0 {
                Cell::Unfilled
            } else {
                Cell::Filled(NonZeroU8::new(digit).unwrap())
            }
        })) {
            *dst = src
        }
        Board { cells }
    }

    pub fn unfilled() -> Board {
        Board {
            cells: [Cell::Unfilled; 81],
        }
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for cell in self.cells.iter() {
            match cell {
                Cell::Unfilled => f.write_str("0")?,
                Cell::Filled(v) => v.fmt(f)?,
            };
        }
        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Unfilled,
    Filled(NonZeroU8),
}
