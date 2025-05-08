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
