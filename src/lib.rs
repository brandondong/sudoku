pub mod create;
pub mod rules;
pub mod solve;

use std::convert::TryInto;
use std::error::Error;
use std::fmt;
use std::num::NonZeroU8;
use std::str::FromStr;

#[derive(Clone, PartialEq)]
pub struct Board {
    // Rows are read from left to right and then top to bottom.
    pub(crate) cells: [Cell; 81],
}

impl FromStr for Board {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 81 {
            return Err(ParseError {});
        }
        let mut cells = [Cell::Unfilled; 81];
        for (dst, src) in cells.iter_mut().zip(s.chars().map(|c| {
            let digit = match c.to_digit(10) {
                None => return Err(ParseError {}),
                Some(v) => v,
            };
            let digit: u8 = digit.try_into().unwrap();
            let cell = match digit.try_into() {
                Ok(v) => Cell::Filled(v),
                Err(_) => Cell::Unfilled,
            };
            Ok(cell)
        })) {
            *dst = src?
        }
        Ok(Self { cells })
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

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    Unfilled,
    Filled(NonZeroU8),
}

#[derive(Debug)]
pub struct ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Parse error")
    }
}

impl Error for ParseError {}
