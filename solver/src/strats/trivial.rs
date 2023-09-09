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

    if grid.has_requirements() {
        for row in grid.iter_by_rows().into_iter() {
            for ((x, y), cell) in row {
                match cell {
                    Indeterminate(_) => {},
                    Requirement(n) | Solution(n) => {
                        changes |= grid.row_requirements[y].insert(n);
                        changes |= grid.col_requirements[x].insert(n);
                    }
                    Blocker(n) => {
                        changes |= grid.row_forbidden[y].insert(n);
                        changes |= grid.col_forbidden[x].insert(n);
                    }
                    Black => {}
                }
            }
        }
    }

    changes
}
