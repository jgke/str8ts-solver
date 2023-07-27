use crate::difficulty::{puzzle_difficulty, Difficulty};
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use rayon::prelude::*;
use solver::grid::{Cell, Grid};
use solver::solver::SolveResults::OutOfBasicStrats;
use solver::solver::{run_fast_basic, solve_round, SolveResults};
use solver::validator::validate;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Task(usize, Grid);

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn remaining_numbers(grid: &Grid) -> usize {
    grid.iter_by_rows()
        .into_iter()
        .flat_map(|row| row.into_iter())
        .map(|(_, cell)| match cell {
            Cell::Requirement(_) => 0,
            Cell::Solution(_) => 0,
            Cell::Blocker(_) => 0,
            Cell::Indeterminate(_) => 1,
            Cell::Black => 0,
        })
        .sum::<usize>()
}

pub fn recur(grid: Grid) -> Option<Grid> {
    let mut queue = BinaryHeap::new();
    let mut max_depth = 0;
    let target_depth = remaining_numbers(&grid);

    queue.push(Task(0, grid));

    while let Some(Task(depth, grid)) = queue.pop() {
        if depth > max_depth {
            max_depth = depth;
            //println!(
            //    "Reached {}/{} (queue={})",
            //    max_depth,
            //    target_depth,
            //    queue.len()
            //);
        }

        if grid.is_solved() {
            return Some(grid);
        }

        let mut cells = grid
            .iter_by_rows()
            .into_iter()
            .flat_map(|row| row.into_iter())
            .filter_map(|((x, y), cell)| match cell {
                Cell::Requirement(_) => None,
                Cell::Solution(_) => None,
                Cell::Blocker(_) => None,
                Cell::Indeterminate(nums) => Some(((x, y), nums)),
                Cell::Black => None,
            })
            .take(1)
            .collect::<Vec<_>>();

        cells.shuffle(&mut thread_rng());

        queue.append(
            &mut cells
                .into_iter()
                .flat_map(|((x, y), nums)| {
                    let vec: Vec<_> = nums
                        .into_iter()
                        .collect::<Vec<_>>()
                        .into_par_iter()
                        .filter_map(|num| {
                            let mut new_grid = grid.clone();
                            new_grid.cells[y][x] = Cell::Requirement(num);
                            while run_fast_basic(&mut new_grid) != OutOfBasicStrats {
                                if validate(&new_grid).is_err() {
                                    break;
                                }
                            }
                            if validate(&new_grid).is_ok() {
                                Some(Task(target_depth - remaining_numbers(&grid), new_grid))
                            } else {
                                None
                            }
                        })
                        .collect();
                    vec.into_iter()
                })
                .collect(),
        );
    }
    None
}

pub fn get_puzzle_difficulty(grid: &Grid) -> Option<Difficulty> {
    let solution = {
        let mut grid = grid.clone();
        let mut history = Vec::new();
        loop {
            match solve_round(&mut grid) {
                Ok(SolveResults::PuzzleSolved) => {
                    break;
                }
                Ok(res) => history.push(res),
                Err(_) => return None,
            }
        }
        history
    };
    let solution = solution.iter().collect::<Vec<_>>();
    Some(puzzle_difficulty(&solution))
}

pub fn remove_numbers(grid: Grid, target_difficulty: usize, symmetric: bool) -> Option<Grid> {
    let size = grid.cells.len();
    let difficulty = get_puzzle_difficulty(&grid)?;
    if difficulty.star_count > target_difficulty {
        return None;
    }

    let mut candidates = grid
        .iter_by_rows()
        .into_iter()
        .flat_map(|row| row.into_iter())
        .filter_map(|((x, y), cell)| match cell {
            Cell::Requirement(_) => None,
            Cell::Solution(_) => Some((x, y)),
            Cell::Blocker(_) => None,
            Cell::Indeterminate(_) => None,
            Cell::Black => None,
        })
        .collect::<Vec<_>>();
    candidates.shuffle(&mut thread_rng());

    for (x, y) in candidates {
        let mut grid = grid.clone();
        grid.cells[y][x] = Cell::Indeterminate((1..=size as u8).collect());
        if symmetric {
            grid.cells[size - y - 1][size - x - 1] =
                Cell::Indeterminate((1..=size as u8).collect());
        }
        if let Some(grid) = remove_numbers(grid, target_difficulty, symmetric) {
            return Some(grid);
        }
    }

    Some(grid)
}

pub fn generator(
    size: usize,
    mut blocker_count: usize,
    mut blocker_num_count: usize,
    target_difficulty: usize,
    symmetric: bool,
) -> Option<(Grid, usize)> {
    let mut grid = Grid::parse(vec![format!("{}", "0".repeat(2 * size * size))]).unwrap();

    let mut blockers = grid
        .iter_by_rows()
        .into_iter()
        .flat_map(|row| row.into_iter())
        .filter(|((x, y), _)| {
            if symmetric {
                *x <= size / 2 && *y <= size / 2
            } else {
                true
            }
        })
        .map(|(pos, _)| pos)
        .collect::<Vec<_>>();
    blockers.shuffle(&mut thread_rng());

    if symmetric {
        blocker_count /= 2;
        blocker_num_count /= 2;
    }
    for (x, y) in blockers.into_iter().take(blocker_count) {
        grid.cells[y][x] = Cell::Black;
        if symmetric {
            grid.cells[size - y - 1][size - x - 1] = Cell::Black;
        }
    }

    let mut open_cells = grid
        .iter_by_rows()
        .into_iter()
        .flat_map(|row| row.into_iter())
        .filter_map(|((x, y), cell)| match cell {
            Cell::Requirement(_) => None,
            Cell::Solution(_) => None,
            Cell::Blocker(_) => None,
            Cell::Indeterminate(_) => Some((x, y)),
            Cell::Black => None,
        })
        .collect::<Vec<_>>();
    open_cells.shuffle(&mut thread_rng());

    let mut blocker_cells = grid
        .iter_by_rows()
        .into_iter()
        .flat_map(|row| row.into_iter())
        .filter(|((x, y), _)| {
            if symmetric {
                *x <= size / 2 && *y <= size / 2
            } else {
                true
            }
        })
        .filter_map(|((x, y), cell)| match cell {
            Cell::Requirement(_) => None,
            Cell::Solution(_) => None,
            Cell::Blocker(_) => None,
            Cell::Indeterminate(_) => None,
            Cell::Black => Some((x, y)),
        })
        .collect::<Vec<_>>();
    blocker_cells.shuffle(&mut thread_rng());

    for (x, y) in blocker_cells.into_iter().take(blocker_num_count) {
        let n = thread_rng().gen_range(1..=size);
        grid.cells[y][x] = Cell::Blocker(n as u8);
        if symmetric {
            let n2 = thread_rng().gen_range(1..=size);
            grid.cells[size - y - 1][size - x - 1] = Cell::Blocker(n2 as u8);
        }
    }

    validate(&grid).ok()?;

    //println!("Attempting to generate\n{}", grid);

    let grid = recur(grid)?;

    //println!("\nSolved grid:\n{}", grid);
    let final_grid = remove_numbers(grid, target_difficulty, symmetric).unwrap();
    let difficulty = get_puzzle_difficulty(&final_grid).unwrap().star_count;
    //println!("Final difficulty: {}", difficulty);
    Some((final_grid, difficulty))
}
