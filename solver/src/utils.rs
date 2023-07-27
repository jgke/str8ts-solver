#[cfg(test)]
use crate::bitset::BitSet;
#[cfg(test)]
use crate::grid::Cell::*;
#[cfg(test)]
use crate::grid::{Cell, Grid};

#[cfg(test)]
pub fn g(grid: &str) -> Grid {
    Grid::parse(grid.trim().lines().map(|row| row.to_string()).collect()).unwrap()
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
