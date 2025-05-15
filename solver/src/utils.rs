#[cfg(test)]
use crate::bitset::BitSet;
#[cfg(test)]
use crate::grid::Cell::*;
#[cfg(test)]
use crate::grid::{Cell, Grid, Point};

#[cfg(test)]
pub fn g(grid: &str) -> Grid {
    Grid::parse_oneline(grid).unwrap()
}
#[cfg(test)]
pub fn set<const N: usize>(vals: [u8; N]) -> BitSet {
    #[cfg(test)]
    vals.into_iter().collect()
}
#[cfg(test)]
pub fn det<const N: usize>(vals: [u8; N]) -> Cell {
    Indeterminate(vals.into_iter().collect())
}

#[cfg(test)]
pub fn set_range<const N: usize>(grid: &mut Grid, tl: Point, br: Point, vals: [u8; N]) {
    for y in tl.1..=br.1 {
        for x in tl.0..=br.0 {
            grid.cells[y][x] = det(vals);
        }
    }
}

// Taken from: https://github.com/rust-lang/rust/blob/master/library/coretests/tests/num/mod.rs
// Not currently in std lib (issue: #27728)
pub fn format_radix<T>(mut x: T, radix: T) -> String
where
    T: std::ops::Rem<Output = T>,
    T: std::ops::Div<Output = T>,
    T: std::cmp::PartialEq,
    T: std::default::Default,
    T: Copy,
    T: Default,
    u32: TryFrom<T>,
{
    let mut result = vec![];

    loop {
        let m = x % radix;
        x = x / radix;
        result.push(std::char::from_digit(m.try_into().ok().unwrap(), radix.try_into().ok().unwrap()).unwrap());
        if x == T::default() {
            break;
        }
    }
    result.into_iter().rev().collect()
}
