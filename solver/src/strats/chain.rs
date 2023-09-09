use crate::bitset::BitSet;
use crate::grid::Cell::*;
use crate::grid::Grid;
use crate::solver::SolveResults::*;
use crate::solver::{run_basic, SolveResults};
use crate::validator::validate;
use itertools::Itertools;
use std::collections::VecDeque;

type ForcedNumber = ((usize, usize), u8);

fn run_chain(candidates: Vec<(Grid, ForcedNumber)>, max_depth: usize) -> Option<ForcedNumber> {
    let mut candidates: VecDeque<((Grid, ForcedNumber), usize)> =
        candidates.into_iter().map(|c| (c, 0)).collect();

    while let Some(((mut temp_grid, (pos, n)), count)) = candidates.pop_front() {
        if count > max_depth {
            continue;
        }
        match run_basic(&mut temp_grid) {
            Ok(OutOfBasicStrats) => {}
            Ok(PuzzleSolved) => {} // well...
            Ok(_) => {
                if validate(&temp_grid).is_err() {
                    return Some((pos, n));
                }
                candidates.push_back(((temp_grid, (pos, n)), count + 1));
            }
            Err(_) => return Some((pos, n)),
        }
    }

    None
}

pub fn gather_and_run_chain<F>(grid: &Grid, filter: F, max_depth: usize) -> Option<ForcedNumber>
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

#[allow(clippy::type_complexity)]
pub fn chain(grid: &mut Grid) -> Option<((usize, usize), u8, Vec<(Grid, SolveResults)>, Grid)> {
    let mut temp_grid = grid.clone();
    let num_count = grid.x;

    if let Some(((x, y), n)) = {
        (2..=num_count)
            .filter_map(|set_size| gather_and_run_chain(grid, |set| set.len() == set_size, 8))
            .chain((2..=num_count).filter_map(|set_size| {
                gather_and_run_chain(grid, |set| set.len() == set_size, usize::MAX)
            }))
            .next()
    } {
        grid.set_impossible((x, y), n).unwrap();
        let mut steps = vec![(temp_grid.clone(), StartChain((x, y), n))];

        temp_grid.set_cell((x, y), Solution(n));

        loop {
            let prev_grid = temp_grid.clone();
            match run_basic(&mut temp_grid) {
                Ok(OutOfBasicStrats) => unreachable!(),
                Ok(PuzzleSolved) => unreachable!(),
                Ok(step) => {
                    steps.push((prev_grid, step));
                    if let Err(e) = validate(&temp_grid) {
                        steps.push((temp_grid.clone(), EndChain(e)));
                        return Some(((x, y), n, steps, temp_grid));
                    }
                }
                Err(e) => {
                    steps.push((temp_grid.clone(), EndChain(e)));
                    return Some(((x, y), n, steps, temp_grid));
                }
            }
        }
    };
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solver::solve_basic;
    use crate::solver::SolveResults::OutOfBasicStrats;
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
        assert!(res.is_some());
        let (pos, n, _, _) = res.unwrap();
        assert_eq!((1, 0), pos);
        assert_eq!(4, n);

        assert_eq!(grid.get_cell((1, 0)), &det([2, 3, 5]));
    }
}
