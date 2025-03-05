use crate::bitset::BitSet;
use crate::grid::Cell::*;
use crate::grid::Grid;
use crate::solver::SolveResults::*;
use crate::solver::{solve_round, SolveResults, ValidationResult};
use crate::strats::gather_implicator_set;
use crate::validator::validate;
use itertools::Itertools;
use std::collections::VecDeque;

type ForcedNumber = ((usize, usize), u8);

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ChainResult {
    Error(ForcedNumber),
    NotUnique(ForcedNumber),
}

fn run_chain(candidates: Vec<(Grid, ForcedNumber)>, max_depth: usize) -> Option<ChainResult> {
    let mut candidates: VecDeque<((Grid, ForcedNumber), usize)> =
        candidates.into_iter().map(|c| (c, 0)).collect();

    while let Some(((mut temp_grid, (pos, n)), count)) = candidates.pop_front() {
        if count > max_depth {
            continue;
        }
        match solve_round(&mut temp_grid, false) {
            Err(ValidationResult::OutOfStrats) => {
                if crate::strats::is_ambiguous(&temp_grid).is_some() {
                    return Some(ChainResult::NotUnique((pos, n)));
                }
            }
            Ok(PuzzleSolved) => {} // well...
            Ok(_) => {
                if validate(&temp_grid).is_err() {
                    return Some(ChainResult::Error((pos, n)));
                }
                if crate::strats::is_ambiguous(&temp_grid).is_some() {
                    return Some(ChainResult::NotUnique((pos, n)));
                }
                candidates.push_back(((temp_grid, (pos, n)), count + 1));
            }
            Err(_) => return Some(ChainResult::Error((pos, n))),
        }
    }

    None
}

pub fn gather_and_run_chain<F>(grid: &Grid, filter: F, max_depth: usize) -> Option<ChainResult>
where
    F: Fn(BitSet) -> bool,
{
    let mut candidates = Vec::new();

    for row in grid.iter_by_rows() {
        for (pos, cell) in row {
            if let Indeterminate(set) = cell {
                if filter(set) {
                    for n in set.into_iter().sorted() {
                        let mut new_grid = grid.clone();
                        new_grid.set_cell((pos.0, pos.1), Solution(n));
                        candidates.push((new_grid, (pos, n)))
                    }
                }
            }
        }
    }

    run_chain(candidates, max_depth)
}

type ChainRes = ((usize, usize), u8, Vec<(Grid, SolveResults)>, Grid);
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ChainSolveResult {
    Error(ChainRes),
    NotUnique(ChainRes),
}

#[allow(clippy::type_complexity)]
pub fn chain(grid: &mut Grid) -> Result<Option<ChainSolveResult>, ValidationResult> {
    let mut temp_grid = grid.clone();
    let num_count = grid.x;

    if let Some(res) = {
        (2..=3)
            .filter_map(|set_size| gather_and_run_chain(grid, |set| set.len() == set_size, 25))
            .chain((2..=num_count).filter_map(|set_size| {
                gather_and_run_chain(grid, |set| set.len() == set_size, usize::MAX)
            }))
            .next()
    } {
        let ((x, y), n) = match res {
            ChainResult::Error(res) => res,
            ChainResult::NotUnique(res) => res,
        };
        grid.set_impossible((x, y), n)?;
        let mut steps = vec![(temp_grid.clone(), StartChain((x, y), n))];

        temp_grid.set_cell((x, y), Solution(n));

        loop {
            let prev_grid = temp_grid.clone();
            match solve_round(&mut temp_grid, false) {
                Err(ValidationResult::OutOfStrats) => {
                    if let Some(pos) = crate::strats::is_ambiguous(&temp_grid) {
                        steps.push((
                            temp_grid.clone(),
                            EndChain(ValidationResult::Ambiguous {
                                cells: gather_implicator_set(&temp_grid, pos)
                                    .into_iter()
                                    .sorted()
                                    .collect(),
                            }),
                        ));
                        return Ok(Some(ChainSolveResult::NotUnique((
                            (x, y),
                            n,
                            steps,
                            temp_grid,
                        ))));
                    }
                    unreachable!();
                }
                Ok(PuzzleSolved) => unreachable!(),
                Ok(step) => {
                    steps.push((prev_grid, step));
                    if let Err(e) = validate(&temp_grid) {
                        steps.push((temp_grid.clone(), EndChain(e)));
                        return Ok(Some(ChainSolveResult::Error(((x, y), n, steps, temp_grid))));
                    }
                    if let Some(pos) = crate::strats::is_ambiguous(&temp_grid) {
                        steps.push((
                            temp_grid.clone(),
                            EndChain(ValidationResult::Ambiguous {
                                cells: gather_implicator_set(&temp_grid, pos)
                                    .into_iter()
                                    .sorted()
                                    .collect(),
                            }),
                        ));
                        return Ok(Some(ChainSolveResult::NotUnique((
                            (x, y),
                            n,
                            steps,
                            temp_grid,
                        ))));
                    }
                }
                Err(e) => {
                    steps.push((temp_grid.clone(), EndChain(e)));
                    return Ok(Some(ChainSolveResult::Error(((x, y), n, steps, temp_grid))));
                }
            }
        }
    };
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solver::solve_basic;
    use crate::strats::{setti, update_required_and_forbidden};
    use crate::utils::*;

    #[test]
    fn test_chain() {
        let mut grid = g("
..1..
5#...
.##.5
.....
.....
");

        assert_eq!(solve_basic(&mut grid), Ok(OutOfBasicStrats));
        assert_eq!(update_required_and_forbidden(&mut grid), Ok(true));
        assert_eq!(setti(&mut grid), Some(set([5])));
        assert_eq!(solve_basic(&mut grid), Ok(OutOfBasicStrats));

        let res = chain(&mut grid);
        let (pos, n, _, _) = match res {
            Ok(Some(crate::strats::ChainSolveResult::Error(res))) => res,
            _ => unreachable!(),
        };
        assert_eq!((0, 2), pos);
        assert_eq!(3, n);

        assert_eq!(grid.get_cell((3, 0)), &det([2, 3, 5]));
    }
}
