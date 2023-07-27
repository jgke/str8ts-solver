use crate::grid::Cell::*;
use crate::grid::Grid;

pub fn trivial(grid: &mut Grid) -> bool {
    let mut changes = false;
    for row in grid.iter_by_rows().into_iter() {
        for ((x, y), cell) in row {
            match cell {
                Indeterminate(set) => {
                    if set.len() == 1 {
                        grid.cells[y][x] = Solution(set.into_iter().next().unwrap());
                        changes = true;
                    }
                }
                Requirement(_) | Solution(_) | Blocker(_) | Black => {}
            }
        }
    }

    changes
}
