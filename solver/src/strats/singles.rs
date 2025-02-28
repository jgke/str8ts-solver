use crate::grid::Cell::*;
use crate::grid::Grid;
use crate::solver::ValidationResult;
use crate::strats::required_in_compartment_by_range;

pub fn singles(grid: &mut Grid) -> Result<bool, ValidationResult> {
    let mut changes = false;

    for compartment in grid.iter_by_compartments().into_iter().flatten() {
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

    Ok(changes)
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
        assert_eq!(singles(&mut grid), Ok(true));
        assert_eq!(grid.cells[2][0], Solution(3));
        assert_eq!(grid.cells[0][2], Solution(3));
    }
}
