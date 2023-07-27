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
            OutOfBasicStrats => {}
            PuzzleSolved => {} // well...
            _ => {
                if validate(&temp_grid).is_err() {
                    return Some((pos, n));
                }
                candidates.push_back(((temp_grid, (pos, n)), count + 1));
            }
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
                        new_grid.cells[pos.1][pos.0] = Solution(n);
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
    let num_count = grid.cells.len();

    if let Some(((x, y), n)) = {
        (2..=num_count)
            .filter_map(|set_size| gather_and_run_chain(grid, |set| set.len() == set_size, 8))
            .chain((2..=num_count).filter_map(|set_size| {
                gather_and_run_chain(grid, |set| set.len() == set_size, usize::MAX)
            }))
            .next()
    } {
        grid.set_impossible((x, y), n);
        let mut steps = vec![(temp_grid.clone(), StartChain((x, y), n))];

        temp_grid.cells[y][x] = Solution(n);

        loop {
            let prev_grid = temp_grid.clone();
            match run_basic(&mut temp_grid) {
                OutOfBasicStrats => unreachable!(),
                PuzzleSolved => unreachable!(),
                step => {
                    steps.push((prev_grid, step));
                    if let Err(e) = validate(&temp_grid) {
                        steps.push((temp_grid.clone(), EndChain(e)));
                        return Some(((x, y), n, steps, temp_grid));
                    }
                }
            }
        }
    };
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solver::run_basic;
    use crate::solver::SolveResults::OutOfBasicStrats;
    use crate::strats::{setti, setti_min_max, update_required_and_forbidden};
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

        while run_basic(&mut grid) != OutOfBasicStrats {}
        assert!(update_required_and_forbidden(&mut grid));
        assert!(setti(&mut grid));
        assert!(setti_min_max(&mut grid));
        while run_basic(&mut grid) != OutOfBasicStrats {}

        let res = chain(&mut grid);
        assert!(res.is_some());
        let (pos, n, _, _) = res.unwrap();
        assert_eq!((1, 0), pos);
        assert_eq!(4, n);

        assert_eq!(grid.cells[0][1], det([2, 3, 5]));
    }
}
