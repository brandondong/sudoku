pub mod create;
pub mod rules;
pub mod solve;

use std::convert::TryInto;
use std::error::Error;
use std::fmt;
use std::num::NonZeroU8;
use std::str::FromStr;

#[derive(Clone, PartialEq)]
pub struct Board<
    const NUM_CELLS: usize,
    const LENGTH: usize,
    const BOX_WIDTH: usize,
    const BOX_HEIGHT: usize,
> {
    // Rows are read from left to right and then top to bottom.
    pub(crate) cells: [Cell; NUM_CELLS],
}

impl<
        const NUM_CELLS: usize,
        const LENGTH: usize,
        const BOX_WIDTH: usize,
        const BOX_HEIGHT: usize,
    > Board<NUM_CELLS, LENGTH, BOX_WIDTH, BOX_HEIGHT>
{
    pub fn unfilled() -> Self {
        Board {
            cells: [Cell::Unfilled; NUM_CELLS],
        }
    }
}

impl<
        const NUM_CELLS: usize,
        const LENGTH: usize,
        const BOX_WIDTH: usize,
        const BOX_HEIGHT: usize,
    > FromStr for Board<NUM_CELLS, LENGTH, BOX_WIDTH, BOX_HEIGHT>
{
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != NUM_CELLS {
            return Err(ParseError {});
        }
        let mut cells = [Cell::Unfilled; NUM_CELLS];
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

impl<
        const NUM_CELLS: usize,
        const LENGTH: usize,
        const BOX_WIDTH: usize,
        const BOX_HEIGHT: usize,
    > fmt::Debug for Board<NUM_CELLS, LENGTH, BOX_WIDTH, BOX_HEIGHT>
{
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

#[derive(Debug)]
pub struct ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Parse error")
    }
}

impl Error for ParseError {}
