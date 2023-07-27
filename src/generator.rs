use crate::difficulty::{puzzle_difficulty, Difficulty};
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use rayon::prelude::*;
use rustc_hash::FxHashSet;
use solver::grid::{Cell, Grid};
use solver::solver::SolveResults::OutOfBasicStrats;
use solver::solver::{run_fast_basic, solve_round, SolveResults};
use solver::validator::validate;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Eq, PartialEq)]
struct Task<T>(T, Grid);

impl<T: Ord> Ord for Task<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl<T: Ord> PartialOrd for Task<T> {
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
                    let nums = nums.into_iter().collect::<Vec<_>>();
                    let mut vec: Vec<_> = nums
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
                    vec.shuffle(&mut thread_rng());
                    vec.into_iter()
                })
                .collect(),
        );
    }
    None
}

pub fn get_puzzle_difficulty(grid: &Grid, enable_chains: bool) -> Option<Difficulty> {
    let solution = {
        let mut grid = grid.clone();
        let mut history = Vec::new();
        loop {
            match solve_round(&mut grid, enable_chains) {
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

fn get_grid_hash(grid: &Grid) -> u128 {
    let mut hash = 0;
    for ((x, y), cell) in grid.iter_by_cells() {
        match cell {
            Cell::Requirement(_) => {}
            Cell::Solution(_) => {}
            Cell::Blocker(_) => {}
            Cell::Indeterminate(_) => {
                hash |= 1 << (y * grid.x + x);
            }
            Cell::Black => {}
        }
    }
    hash
}

pub fn remove_numbers(grid: Grid, target_difficulty: usize, symmetric: bool) -> Option<Grid> {
    let mut queue: BinaryHeap<Task<(usize, usize)>> = BinaryHeap::new();
    queue.push(Task((0, 0), grid.clone()));
    let size = grid.cells.len();

    let diff = get_puzzle_difficulty(&grid, target_difficulty >= 6).unwrap();
    let best_difficulty = Arc::new(Mutex::new((diff.star_count, diff.move_count)));
    let best_grid = Arc::new(Mutex::new(grid));
    let mut iterations = 0;

    let seen = Arc::new(Mutex::new(FxHashSet::<u128>::default()));

    while !queue.is_empty() {
        iterations += 1;
        {
            println!(
                "queue len={} iter={} seen elements={}",
                queue.len(),
                iterations,
                seen.lock().unwrap().len()
            );
        }

        let mut pool = Vec::new();
        for _ in 0..rayon::current_num_threads() {
            if let Some(item) = queue.pop() {
                pool.push(item);
            }
        }

        let mut next_candidates = pool
            .into_par_iter()
            .flat_map(|Task((star_count, move_count), grid)| {
                {
                    let mut best_difficulty = best_difficulty.lock().unwrap();
                    if star_count > target_difficulty
                        || star_count < best_difficulty.0
                        || (star_count == best_difficulty.0 && move_count + 5 < best_difficulty.1)
                    {
                        return vec![].into_par_iter();
                    } else if star_count > best_difficulty.0
                        || (star_count == best_difficulty.0 && move_count > best_difficulty.1)
                    {
                        println!("New best, difficulty = {}:\n{}", star_count, grid);

                        *best_difficulty = (star_count, move_count);
                        *best_grid.lock().unwrap() = grid.clone();
                    }
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

                candidates
                    .into_par_iter()
                    .filter_map(|(x, y)| {
                        let mut grid = grid.clone();
                        grid.cells[y][x] = Cell::Indeterminate((1..=size as u8).collect());
                        if symmetric {
                            grid.cells[size - y - 1][size - x - 1] =
                                Cell::Indeterminate((1..=size as u8).collect());
                        }
                        {
                            let grid_hash = get_grid_hash(&grid);
                            let mut seen = seen.lock().unwrap();
                            if seen.contains(&grid_hash) {
                                return None;
                            }
                            seen.insert(grid_hash);
                        }
                        get_puzzle_difficulty(&grid, target_difficulty >= 6).map(|difficulty| {
                            Task((difficulty.star_count, difficulty.move_count), grid)
                        })
                    })
                    .collect::<Vec<_>>()
                    .into_par_iter()
            })
            .collect();

        queue.append(&mut next_candidates)
    }
    let x = best_grid.lock().unwrap().clone();
    Some(x)
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

    println!("Attempting to generate\n{}", grid);

    let grid = recur(grid)?;

    println!("\nSolved grid:\n{}", grid);
    let final_grid = remove_numbers(grid, target_difficulty, symmetric).unwrap();
    println!("Calculating final difficulty");
    let difficulty = get_puzzle_difficulty(&final_grid, true).unwrap().star_count;
    println!("Final difficulty: {}", difficulty);
    Some((final_grid, difficulty))
}
