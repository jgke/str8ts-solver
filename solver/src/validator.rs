use crate::bitset::BitSet;
use crate::grid::Cell::*;
use crate::grid::{Cell, Compartment, Grid};
use crate::solver::ValidationResult;
use crate::solver::ValidationResult::*;
use rustc_hash::FxHashMap;

pub fn cell_has_solutions(x: usize, y: usize, cell: &Cell) -> Result<(), ValidationResult> {
    match cell {
        Requirement(_) | Solution(_) | Blocker(_) | Black => {}
        Indeterminate(group) => {
            if group.is_empty() {
                return Err(EmptyCell { pos: (x, y) });
            }
        }
    }
    Ok(())
}

pub fn grid_has_conflicts(grid: &Grid) -> Result<(), ValidationResult> {
    for (_, row) in grid.iter_by_rows_and_cols() {
        let mut map = FxHashMap::default();
        for ((x, y), cell) in row {
            if let Some(val) = cell.to_determinate() {
                if map.contains_key(&val) {
                    let (other_x, other_y) = map[&val];
                    return Err(Conflict {
                        pos1: (other_x + 1, other_y + 1),
                        pos2: (x + 1, y + 1),
                        val,
                    });
                }
                map.insert(val, (x, y));
            }
        }
    }
    Ok(())
}

pub fn compartment_valid(compartment: &Compartment) -> Result<(), ValidationResult> {
    let (x, y) = compartment.cells[0].0;
    let vertical = compartment.vertical;
    let definite_nums: BitSet = compartment
        .cells
        .iter()
        .flat_map(|(_, cell)| cell.to_determinate())
        .collect();
    let available_nums: BitSet = compartment
        .cells
        .iter()
        .flat_map(|(_, cell)| cell.to_possibles().into_iter())
        .collect();

    if definite_nums.is_empty() {
        return Ok(());
    }

    let top_left = (x, y);
    let size = compartment.cells.len() as u8;
    let min = definite_nums.into_iter().min().unwrap();
    let max = definite_nums.into_iter().max().unwrap();

    for n in min..=max {
        if !available_nums.contains(n) {
            return Err(Sequence {
                vertical,
                top_left,
                range: (min, max),
                missing: n,
            });
        }
    }

    if max - min > size {
        return Err(SequenceTooLarge {
            contains: (min, max),
            vertical,
            top_left,
            max_ranges: ((min, min + size - 1), (max + 1 - size, max)),
        });
    }

    Ok(())
}

pub fn validate(grid: &Grid) -> Result<(), ValidationResult> {
    for y in 0..grid.y {
        for x in 0..grid.x {
            cell_has_solutions(x, y, &grid.cells[y][x])?;
        }
    }

    grid_has_conflicts(grid)?;

    for line in grid.iter_by_compartments() {
        for compartment in line {
            compartment_valid(&compartment)?;
        }
    }

    Ok(())
}
