use crate::bitset::BitSet;
use crate::grid::{Cell, Grid};
use crate::solver::ValidationResult;
use crate::strats::get_compartment_range;

fn single_cell_cross_compartment_unique(
    grid: &mut Grid,
    x: usize,
    y: usize,
    set: BitSet,
) -> Result<Option<((usize, usize), u8)>, ValidationResult> {
    let mut free_set = set.clone();
    let (row, col) = grid.compartments_containing((x, y));
    if row
        .cells
        .into_iter()
        .filter(|(_, cell)| matches!(cell, Cell::Indeterminate(_)))
        .count()
        == 1
        && col
            .cells
            .into_iter()
            .filter(|(_, cell)| matches!(cell, Cell::Indeterminate(_)))
            .count()
            == 1
    {
        for (p, c) in grid.get_row(y) {
            if p != (x, y) {
                free_set = free_set.difference(c.to_possibles());
            }
        }
        for (p, c) in grid.get_col(x) {
            if p != (x, y) {
                free_set = free_set.difference(c.to_possibles());
            }
        }

        if free_set.len() > 1 {
            return Err(ValidationResult::Ambiguous {
                cells: vec![(x, y)],
            });
        }
        if let Some(res) = free_set.into_iter().next() {
            grid.set_cell((x, y), Cell::Solution(res));
            return Ok(Some(((x, y), res)));
        }
    }
    Ok(None)
}

fn single_cell_intra_compartment_unique(
    grid: &mut Grid,
    x: usize,
    y: usize,
    set: BitSet,
) -> Result<Option<((usize, usize), u8)>, ValidationResult> {
    let mut free_set = set.clone();
    let (row, col) = grid.compartments_containing((x, y));
    let (minx, maxx) = get_compartment_range(grid.x, &row, None).unwrap();
    let (miny, maxy) = get_compartment_range(grid.y, &col, None).unwrap();
    let range_size = maxx - minx + 1;

    if minx != miny || maxx != maxy {
        return Ok(None);
    }
    if (row.cells.len() as u8 + 1 == range_size || col.cells.len() as u8 + 1 == range_size)
        && free_set.contains(minx)
        && free_set.contains(maxx)
    {
        for (p, c) in grid.get_row(y) {
            if !row.contains_pos(p) {
                free_set = free_set.difference(c.to_possibles());
            }
        }
        for (p, c) in grid.get_col(x) {
            if !row.contains_pos(p) {
                free_set = free_set.difference(c.to_possibles());
            }
        }

        if free_set.contains(minx) {
            grid.set_impossible((x, y), maxx)?;
            return Ok(Some(((x, y), maxx)));
        }
        if free_set.contains(maxx) {
            grid.set_impossible((x, y), minx)?;
            return Ok(Some(((x, y), minx)));
        }
    }
    Ok(None)
}

pub fn unique_requirement(
    grid: &mut Grid,
) -> Result<Option<((usize, usize), bool, u8)>, ValidationResult> {
    for ((x, y), cell) in grid.iter_by_cells() {
        if let Cell::Indeterminate(set) = cell {
            if let Some(((x, y), res)) = single_cell_cross_compartment_unique(grid, x, y, set)? {
                return Ok(Some(((x, y), true, res)));
            }
            if let Some(((x, y), res)) = single_cell_intra_compartment_unique(grid, x, y, set)? {
                return Ok(Some(((x, y), false, res)));
            }
        }
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid::Cell;
    use crate::solver::solve_basic;
    use crate::solver::SolveResults::OutOfBasicStrats;
    use crate::strats::update_required_and_forbidden;
    use crate::utils::*;

    #[test]
    fn test_cross_compartment() {
        let mut grid = g("
#######
##..#.#
#######
##..#.#
##..#.#
#######
#######
");
        grid.cells[1][2] = det([1, 2, 3]);
        grid.cells[1][3] = det([1, 2, 3]);

        grid.cells[1][5] = det([1, 5]);

        grid.cells[3][5] = det([1, 2, 3]);
        grid.cells[4][5] = det([1, 2, 3]);

        grid.cells[3][2] = det([1, 2, 3, 4]);
        grid.cells[3][3] = det([1, 2, 3, 4]);
        grid.cells[4][2] = det([1, 2, 3, 4]);
        grid.cells[4][3] = det([1, 2, 3, 4]);

        assert_eq!(solve_basic(&mut grid), Ok(OutOfBasicStrats));
        assert_eq!(update_required_and_forbidden(&mut grid), Ok(true));
        assert_eq!(solve_basic(&mut grid), Ok(OutOfBasicStrats));

        assert_eq!(unique_requirement(&mut grid), Ok(Some(((5, 1), true, 5))));

        assert_eq!(grid.cells[1][2], det([1, 2, 3]));
        assert_eq!(grid.cells[1][3], det([1, 2, 3]));
        assert_eq!(grid.cells[1][5], Cell::Solution(5));
        assert_eq!(grid.cells[3][5], det([1, 2, 3]));
        assert_eq!(grid.cells[4][5], det([1, 2, 3]));
    }

    #[test]
    fn test_intra_compartment() {
        let mut grid = g("
#########
#...#...#
#...#...#
#.......#
#########
#.......#
#.......#
#.......#
#########
");
        set_range(&mut grid, (1, 1), (3, 3), [1, 2, 3, 4]);
        set_range(&mut grid, (5, 1), (8, 3), [4, 5, 6, 7, 8]);
        set_range(&mut grid, (1, 5), (1, 7), [4, 5, 6, 7]);
        grid.cells[1][1] = det([2, 3, 4]);
        grid.cells[2][2] = det([2, 3, 4]);
        grid.cells[2][3] = det([2, 3, 4]);
        grid.cells[3][1] = det([2, 3, 4]);

        assert_eq!(solve_basic(&mut grid), Ok(OutOfBasicStrats));
        assert_eq!(update_required_and_forbidden(&mut grid), Ok(true));
        assert_eq!(solve_basic(&mut grid), Ok(OutOfBasicStrats));

        assert_eq!(unique_requirement(&mut grid), Ok(Some(((1, 2), false, 4))));

        assert_eq!(grid.cells[2][1], det([1, 2, 3]));
    }
}
