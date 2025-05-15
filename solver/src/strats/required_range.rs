use crate::grid::{Grid, Point};
use crate::solve_result::SolveType::RequiredRange;
use crate::strategy::StrategyReturn;
use crate::strats::required_in_compartment_by_range;
use rustc_hash::FxHashSet;

pub fn required_range(grid: &mut Grid) -> StrategyReturn {
    let mut changes = false;

    for compartment in grid.iter_by_compartments() {
        let compartment_positions: FxHashSet<Point> = compartment.cells.iter().map(|(p, _)| *p).collect();
        let sample_pos = compartment.sample_pos();

        for num in required_in_compartment_by_range(grid.x, &compartment) {
            changes |= grid.set_impossible_in(sample_pos, compartment.vertical, num, &compartment_positions)?;
        }
    }

    if changes {
        Ok(Some(RequiredRange.into()))
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::*;

    #[test]
    fn test_required_range() {
        let mut grid = g("
########
#..##..#
#.####.#
########
########
#.####.#
#..##..#
########
");
        grid.cells[1][1] = det([1, 2]);
        grid.cells[2][1] = det([1, 2]);
        grid.cells[1][2] = det([1, 2]);
        assert_eq!(required_range(&mut grid), Ok(Some(RequiredRange.into())));

        assert_eq!(grid.cells[1][1], det([1, 2]));
        assert_eq!(grid.cells[2][1], det([1, 2]));
        assert_eq!(grid.cells[1][2], det([1, 2]));

        assert_eq!(grid.cells[5][1], det([3, 4, 5, 6, 7, 8]));
        assert_eq!(grid.cells[6][1], det([3, 4, 5, 6, 7, 8]));
        assert_eq!(grid.cells[6][2], det([1, 2, 3, 4, 5, 6, 7, 8]));

        assert_eq!(grid.cells[1][5], det([3, 4, 5, 6, 7, 8]));
        assert_eq!(grid.cells[1][6], det([3, 4, 5, 6, 7, 8]));
        assert_eq!(grid.cells[2][6], det([1, 2, 3, 4, 5, 6, 7, 8]));

        assert_eq!(grid.cells[6][5], det([1, 2, 3, 4, 5, 6, 7, 8]));
        assert_eq!(grid.cells[5][6], det([1, 2, 3, 4, 5, 6, 7, 8]));
        assert_eq!(grid.cells[6][6], det([1, 2, 3, 4, 5, 6, 7, 8]));
    }
}
