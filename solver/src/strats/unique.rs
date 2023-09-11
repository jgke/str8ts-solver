use crate::bitset::BitSet;
use crate::grid::Cell::*;
use crate::grid::Compartment;
use crate::grid::{Cell, CellPair, Grid};
use crate::solver::{run_fast_basic, SolveResults, ValidationResult};
use crate::validator::validate;
use itertools::Itertools;
use rustc_hash::FxHashSet;
use std::collections::VecDeque;

fn num_implications(
    num: u8,
    line: Vec<CellPair>,
    compartment: Compartment,
    requirements: BitSet,
) -> usize {
    let mut implicating_cells = FxHashSet::default();

    if !requirements.contains(num) {
        for (pos, cell) in &compartment.cells {
            if let Some(_) = cell.to_maybe_possibles() {
                implicating_cells.insert(pos);
            }
        }
    }

    for (pos, cell) in &line {
        if let Some(set) = cell.to_maybe_possibles() {
            if set.contains(num) {
                implicating_cells.insert(pos);
            }
        }
    }

    implicating_cells.len()
}

fn cell_implicators_(
    this_set: BitSet,
    definite: BitSet,
    compartment: &Compartment,
    line: Vec<CellPair>,
) -> FxHashSet<(usize, usize)> {
    let mut implicating_cells = FxHashSet::default();

    if !this_set.is_subset(definite) {
        for (pos, cell) in &compartment.cells {
            if let Some(set) = cell.to_maybe_possibles() {
                if !set.intersection(this_set).is_empty() {
                    implicating_cells.insert(*pos);
                }
            }
        }
    }
    for (pos, cell) in line {
        if let Some(set) = cell.to_maybe_possibles() {
            if !set.intersection(this_set).is_empty() {
                implicating_cells.insert(pos);
            }
        }
    }

    implicating_cells
}

fn cell_implicators(grid: &Grid, pos: (usize, usize)) -> FxHashSet<(usize, usize)> {
    let this_set = if let Indeterminate(set) = grid.get_cell(pos) {
        *set
    } else {
        return FxHashSet::default();
    };
    let (hor, vert) = grid.compartments_containing(pos);
    let definite_row = grid.row_requirements[pos.1];
    let definite_col = grid.col_requirements[pos.0];

    let mut implicating_cells =
        cell_implicators_(this_set, definite_row, &hor, grid.get_row(pos.1))
            .union(&cell_implicators_(
                this_set,
                definite_col,
                &vert,
                grid.get_col(pos.0),
            ))
            .copied()
            .collect::<FxHashSet<_>>();

    implicating_cells.remove(&pos);

    implicating_cells
}

pub fn solution_count(mut grid: Grid, mut set: FxHashSet<(usize, usize)>) -> usize {
    while run_fast_basic(&mut grid) != Ok(SolveResults::OutOfBasicStrats) {
        if validate(&grid).is_err() {
            return 0;
        }
    }
    if let Some(pos) = set.iter().sorted().cloned().next() {
        set.remove(&pos);
        match grid.get_cell(pos) {
            Cell::Indeterminate(nums) => nums
                .into_iter()
                .map(|num| {
                    let mut subgrid = grid.clone();
                    subgrid.set_cell(pos, Cell::Solution(num));
                    solution_count(subgrid, set.clone())
                })
                .sum(),
            _ => solution_count(grid, set),
        }
    } else if validate(&grid).is_ok() {
        1
    } else {
        0
    }
}

pub fn is_ambiguous(grid: &Grid) -> Option<(usize, usize)> {
    let mut visited_cells = FxHashSet::default();
    for (pos, _set) in grid.iter_by_indeterminates() {
        if visited_cells.contains(&pos) {
            continue;
        }

        let mut implicator_set = FxHashSet::default();
        let mut queue = VecDeque::new();
        queue.push_back(pos);
        while let Some(pos) = queue.pop_front() {
            if implicator_set.contains(&pos) {
                continue;
            }
            implicator_set.insert(pos);
            for pos in cell_implicators(grid, pos) {
                queue.push_back(pos);
            }
        }
        for pos in &implicator_set {
            visited_cells.insert(*pos);
        }

        if implicator_set.len() > 8 {
            continue;
        }

        if solution_count(grid.clone(), implicator_set) > 1 {
            return Some(pos);
        }
    }
    None
}

pub fn unique(grid: &mut Grid) -> Result<Option<SolveResults>, ValidationResult> {
    /* phase 1: if cell has multiple numbers with 0 implicators, it must be _none_ of those */
    for (pos, set) in grid.iter_by_indeterminates() {
        let row = grid.get_row(pos.1);
        let row_reqs = grid.row_requirements[pos.1];
        let col = grid.get_col(pos.0);
        let col_reqs = grid.col_requirements[pos.1];
        let (hor, vert) = grid.compartments_containing(pos);
        let mut free_nums = BitSet::new();
        for num in set {
            if num_implications(num, row.clone(), hor.clone(), row_reqs) <= 1
                && num_implications(num, col.clone(), vert.clone(), col_reqs) <= 1
            {
                free_nums.insert(num);
            }
        }
        if free_nums.len() >= 2 {
            grid.remove_numbers(pos, free_nums)?;
            return Ok(Some(SolveResults::UniqueFreeNums(pos, free_nums)));
        }
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::*;

    #[test]
    fn test_ambiguous_single() {
        let mut grid = g("
.#.
##.
###
");
        grid.cells[0][0] = det([2, 3, 4]);
        grid.cells[0][2] = det([1, 2]);
        grid.cells[1][2] = det([1, 2]);
        assert_eq!(
            unique(&mut grid),
            Ok(Some(SolveResults::UniqueFreeNums((0, 0), set([3, 4]))))
        );
        assert_eq!(grid.cells[0][0], det([2]));
    }

    #[test]
    fn test_ambiguous() {
        let mut grid = g("
.#.
##.
###
");
        grid.cells[0][0] = det([3, 4]);
        grid.cells[0][2] = det([1, 2]);
        grid.cells[1][2] = det([1, 2]);
        assert_eq!(is_ambiguous(&mut grid), Some((2, 0)));
    }
}
