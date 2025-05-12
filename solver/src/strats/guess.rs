use crate::bitset::BitSet;
use crate::grid::Cell::*;
use crate::grid::{Grid, Point};
use crate::solver::SolveType::*;
use crate::solver::{into_ty, solve_round, SolveResults, ValidationError, ValidationResult};
use itertools::Itertools;
use std::collections::VecDeque;

type ForcedNumber = (Point, u8);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GuessResult(pub ForcedNumber);

fn run_guess(candidates: Vec<(Grid, ForcedNumber)>, max_depth: usize) -> Option<GuessResult> {
    let mut candidates: VecDeque<((Grid, ForcedNumber), usize)> =
        candidates.into_iter().map(|c| (c, 0)).collect();

    while let Some(((mut temp_grid, (pos, n)), count)) = candidates.pop_front() {
        if count > max_depth {
            continue;
        }
        match into_ty(solve_round(&mut temp_grid, false)) {
            Err(ValidationError::OutOfStrats) => {}
            Ok(PuzzleSolved) => {} // well...
            Ok(_) => candidates.push_back(((temp_grid, (pos, n)), count + 1)),
            Err(_) => return Some(GuessResult((pos, n))),
        }
    }

    None
}

pub fn gather_and_run_guess<F>(grid: &Grid, filter: F, max_depth: usize) -> Option<GuessResult>
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

    run_guess(candidates, max_depth)
}

type GuessStepRes = (Point, u8, Vec<(Grid, SolveResults)>, Grid);
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GuessSolveResult(pub GuessStepRes);

#[allow(clippy::type_complexity)]
pub fn guess(grid: &mut Grid) -> Result<Option<GuessSolveResult>, ValidationResult> {
    let mut temp_grid = grid.clone();
    let num_count = grid.x;

    if let Some(res) = {
        (2..=3)
            .filter_map(|set_size| gather_and_run_guess(grid, |set| set.len() == set_size, 25))
            .chain((2..=num_count).filter_map(|set_size| {
                gather_and_run_guess(grid, |set| set.len() == set_size, usize::MAX)
            }))
            .next()
    } {
        let GuessResult(((x, y), n)) = res;
        grid.set_impossible((x, y), n)?;
        temp_grid.set_cell((x, y), Solution(n));
        let mut steps: Vec<(Grid, SolveResults)> =
            vec![(temp_grid.clone(), StartGuess((x, y), n).into())];

        loop {
            match solve_round(&mut temp_grid, false) {
                Err(ValidationResult {
                    ty: ValidationError::OutOfStrats,
                    meta: _,
                }) => {
                    break;
                }
                Ok(SolveResults {
                    ty: PuzzleSolved,
                    meta: _,
                }) => break,
                Ok(step) => {
                    steps.push((temp_grid.clone(), step));
                }
                Err(e) => {
                    steps.push((temp_grid.clone(), EndGuess(e).into()));
                    return Ok(Some(GuessSolveResult(((x, y), n, steps, temp_grid))));
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
    fn test_guess() {
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

        let res = guess(&mut grid);
        let (pos, n, _, _) = match res {
            Ok(Some(GuessSolveResult(res))) => res,
            _ => unreachable!(),
        };
        assert_eq!((0, 2), pos);
        assert_eq!(3, n);

        assert_eq!(grid.get_cell((3, 0)), &det([2, 3, 5]));
    }

    #[test]
    fn guess_ambiguous() {
        let mut grid = g("
..
..
");

        assert_eq!(Ok(None), guess(&mut grid));
    }
}
