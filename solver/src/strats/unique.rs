use crate::bitset::BitSet;
use crate::grid::Cell::*;
use crate::grid::Compartment;
use crate::grid::{Cell, CellPair, Grid};
use crate::solver::{run_fast_basic, SolveResults};
use crate::validator::validate;
use itertools::Itertools;
use rustc_hash::FxHashSet;
use std::collections::VecDeque;

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

pub fn solution_count(mut grid: Grid, mut set: Vec<(usize, usize)>) -> usize {
    while run_fast_basic(&mut grid) != Ok(SolveResults::OutOfBasicStrats) {
        if validate(&grid).is_err() {
            return 0;
        }
    }
    if let Some(pos) = set.pop() {
        match grid.get_cell(pos) {
            Cell::Indeterminate(nums) => {
                let mut count = 0;
                for num in *nums {
                    if count > 1 {
                        return count;
                    }
                    let mut subgrid = grid.clone();
                    subgrid.set_cell(pos, Cell::Solution(num));
                    count += solution_count(subgrid, set.clone());
                }
                count
            }
            _ => solution_count(grid, set),
        }
    } else if validate(&grid).is_ok() {
        1
    } else {
        0
    }
}

pub fn gather_implicator_set(grid: &Grid, pos: (usize, usize)) -> FxHashSet<(usize, usize)> {
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
    implicator_set
}

fn implicator_difficulty(grid: &Grid, set: &FxHashSet<(usize, usize)>) -> usize {
    let mut diff: usize = 1;
    for pos in set {
        let mul: usize = grid
            .get_cell(*pos)
            .to_maybe_possibles()
            .map(|set| set.len())
            .unwrap_or(1);
        diff = diff.saturating_mul(mul);
    }
    diff
}

pub fn is_ambiguous(grid: &Grid) -> Option<(usize, usize)> {
    let mut visited_cells = FxHashSet::default();
    for (pos, _set) in grid.iter_by_indeterminates() {
        if visited_cells.contains(&pos) {
            continue;
        }

        let implicator_set = gather_implicator_set(grid, pos);
        for pos in &implicator_set {
            visited_cells.insert(*pos);
        }

        if implicator_difficulty(grid, &implicator_set) > 10000 {
            continue;
        }

        let implicator_queue: Vec<_> = implicator_set
            .into_iter()
            .sorted_by_key(|pos| -(grid.get_cell(*pos).to_possibles().len() as isize))
            .collect();
        if solution_count(grid.clone(), implicator_queue) > 1 {
            return Some(pos);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::*;

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
