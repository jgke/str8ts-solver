use crate::bitset::BitSet;
use crate::grid::{Cell::*, CellPair, Grid};
use crate::solver::ValidationResult;
use crate::strats::required_by_range;

pub fn required_by_certain(line: &[CellPair]) -> BitSet {
    let mut required = BitSet::default();

    for compartment in Grid::line_to_compartments(false, line.to_vec()) {
        for (_, cell) in compartment.cells {
            match cell {
                Requirement(n) => {
                    required.insert(n);
                }
                Solution(n) => {
                    required.insert(n);
                }
                Blocker(_) => {}
                Indeterminate(_) => {}
                Black => {}
            }
        }
    }

    required
}

pub fn required_numbers(grid: &Grid, line: &[CellPair]) -> BitSet {
    required_by_certain(line)
        .into_iter()
        .chain(required_by_range(grid.x, line))
        .collect()
}

pub fn forbidden_by_certain(line: &[CellPair]) -> BitSet {
    let mut required = BitSet::default();

    for (_, cell) in line {
        if let Blocker(n) = cell {
            required.insert(*n);
        }
    }

    required
}

pub fn forbidden_numbers(_grid: &Grid, line: &[CellPair]) -> BitSet {
    forbidden_by_certain(line)
}

pub fn update_required_and_forbidden(grid: &mut Grid) -> Result<bool, ValidationResult> {
    let mut changes = false;

    for n in 1..=grid.x as u8 {
        for (vertical, row) in grid.iter_by_rows_and_cols() {
            let sample_pos = row[0].0;

            if required_numbers(grid, &row).contains(n) {
                changes |= grid.requirements_mut(vertical, sample_pos).insert(n);
            }
            if forbidden_numbers(grid, &row).contains(n) {
                changes |= grid.forbidden_mut(vertical, sample_pos).insert(n);
            }
        }
    }

    Ok(changes)
}
