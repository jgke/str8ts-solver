use crate::grid::Grid;
use crate::strats::{forbidden_numbers, required_numbers};

pub fn update_required_and_forbidden(grid: &mut Grid) -> bool {
    let mut changes = false;

    for n in 1..=grid.x as u8 {
        for (y, row) in grid.iter_by_rows().into_iter().enumerate() {
            if required_numbers(grid, &row).contains(n) {
                changes |= grid.row_requirements[y].insert(n);
            }
            if forbidden_numbers(grid, &row).contains(n) {
                changes |= grid.row_forbidden[y].insert(n);
            }
        }
        for (x, col) in grid.iter_by_cols().into_iter().enumerate() {
            if required_numbers(grid, &col).contains(n) {
                changes |= grid.col_requirements[x].insert(n);
            }
            if forbidden_numbers(grid, &col).contains(n) {
                changes |= grid.col_forbidden[x].insert(n);
            }
        }
    }

    changes
}
