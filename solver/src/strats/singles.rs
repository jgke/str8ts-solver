use crate::grid::Cell::*;
use crate::grid::Grid;
use crate::solve_result::SolveType::Singles;
use crate::strategy::StrategyReturn;
use crate::strats::required_in_compartment_by_range;

pub fn singles(grid: &mut Grid) -> StrategyReturn {
    let mut changes = false;

    for compartment in grid.iter_by_compartments() {
        for num in required_in_compartment_by_range(grid.x, &compartment) {
            let mut count = 0;
            let mut sample = None;
            for ((x, y), cell) in &compartment.cells {
                if cell.to_unresolved().contains(num) {
                    count += 1;
                    sample = Some((*x, *y));
                }
            }

            if count == 1 {
                let (x, y) = sample.unwrap();
                grid.set_cell((x, y), Solution(num));
                changes = true;
            }
        }
    }

    if changes {
        Ok(Some(Singles.into()))
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::*;

    #[test]
    fn test_stranded() {
        let mut grid = g("
...
.##
.##

");
        grid.cells[0][0] = det([1, 2]);
        grid.cells[1][0] = det([1, 2]);
        grid.cells[2][0] = det([1, 2, 3]);
        grid.cells[0][1] = det([1, 2]);
        grid.cells[0][2] = det([1, 2, 3]);
        assert_eq!(singles(&mut grid), Ok(Some(Singles.into())));
        assert_eq!(grid.cells[2][0], Solution(3));
        assert_eq!(grid.cells[0][2], Solution(3));
    }
}
