use crate::grid::Cell::*;
use crate::grid::Grid;
use crate::bitset::BitSet;

pub fn trivial(grid: &mut Grid) -> bool {
    let mut changes = false;
    for row in grid.iter_by_rows().into_iter() {
        for ((x, y), cell) in row {
            match cell {
                Indeterminate(set) => {
                    if set.len() == 1 {
                        grid.set_cell((x, y), Solution(set.into_iter().next().unwrap()));
                        changes = true;
                    }
                }
                Requirement(_) | Solution(_) | Blocker(_) | Black => {}
            }
        }
    }

    if grid.has_requirements() {
        for (y, row) in grid.iter_by_rows().into_iter().enumerate() {
            let mut missing_numbers: BitSet = (1..=grid.x as u8).collect();
            for (_, cell) in row {
                match cell {
                    Indeterminate(set) => {for n in set { missing_numbers.remove(n); } }
                    Requirement(n) | Solution(n) => {
                        missing_numbers.remove(n);
                        changes |= grid.row_requirements[y].insert(n);
                    }
                    Blocker(n) => {
                        changes |= grid.row_forbidden[y].insert(n);
                    }
                    Black => {}
                }
            }
            grid.row_forbidden[y].append(missing_numbers);
        }
        for (x, col) in grid.iter_by_cols().into_iter().enumerate() {
            let mut missing_numbers: BitSet = (1..=grid.x as u8).collect();
            for (_, cell) in col {
                match cell {
                    Indeterminate(set) => {for n in set { missing_numbers.remove(n); } }
                    Requirement(n) | Solution(n) => {
                        missing_numbers.remove(n);
                        changes |= grid.col_requirements[x].insert(n);
                    }
                    Blocker(n) => {
                        changes |= grid.col_forbidden[x].insert(n);
                    }
                    Black => {}
                }
            }
            grid.col_forbidden[x].append(missing_numbers);
        }
    }

    changes
}
