use crate::difficulty::get_puzzle_difficulty;
use crate::grid::{Cell, Grid};
use crate::solver::run_fast_basic;
use crate::solver::SolveResults::OutOfBasicStrats;
use crate::validator::validate;
use log::debug;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use rand::{thread_rng, Rng};
use rayon::prelude::*;
use rustc_hash::FxHashSet;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
struct Task<T, R>(T, R, Grid);

impl<T: Ord, R> Eq for Task<T, R> {}

impl<T: Ord, R> PartialEq<Self> for Task<T, R> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0) && self.2.eq(&other.2)
    }
}

impl<T: Ord, R> Ord for Task<T, R> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl<T: Ord, R> PartialOrd for Task<T, R> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn remaining_numbers(grid: &Grid) -> usize {
    grid.iter_by_cell_pos_matching(|cell| matches!(cell, Cell::Indeterminate(_)))
        .len()
}

pub fn recur<Rand: Rng + Send + Clone>(grid: Grid, rng: &mut Rand) -> Option<Grid> {
    let mut queue = BinaryHeap::new();
    let mut max_depth = 0;
    let target_depth = remaining_numbers(&grid);

    queue.push(Task(0, rng.clone(), grid));

    while let Some(Task(depth, mut rng, grid)) = queue.pop() {
        if depth > max_depth {
            max_depth = depth;
            debug!(
                "Reached {}/{} (queue={})",
                max_depth,
                target_depth,
                queue.len()
            );
        }

        if grid.is_solved() {
            return Some(grid);
        }

        let ((x, y), nums) = grid
            .iter_by_cells()
            .into_iter()
            .filter_map(|((x, y), cell)| match cell {
                Cell::Requirement(_) => None,
                Cell::Solution(_) => None,
                Cell::Blocker(_) => None,
                Cell::Indeterminate(nums) => Some(((x, y), nums)),
                Cell::Black => None,
            })
            .next()
            .unwrap();

        let nums = nums
            .into_iter()
            .map(|num| (num, rng.clone()))
            .collect::<Vec<_>>();
        let mut vec: Vec<_> = nums
            .into_par_iter()
            .filter_map(|(num, rng)| {
                let mut new_grid = grid.clone();
                new_grid.cells[y][x] = Cell::Requirement(num);
                while run_fast_basic(&mut new_grid) != OutOfBasicStrats {
                    if validate(&new_grid).is_err() {
                        break;
                    }
                }
                if validate(&new_grid).is_ok() {
                    Some(Task(target_depth - remaining_numbers(&grid), rng, new_grid))
                } else {
                    None
                }
            })
            .collect();
        vec.shuffle(&mut rng);
        queue.append(&mut vec.into_iter().collect());
    }
    None
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

pub fn remove_numbers<Rand: Rng + Send + Clone>(
    grid: Grid,
    target_difficulty: usize,
    symmetric: bool,
    rng: &mut Rand,
) -> Option<Grid> {
    let mut queue: BinaryHeap<Task<(usize, usize), Rand>> = BinaryHeap::new();
    queue.push(Task((0, 0), rng.clone(), grid.clone()));
    let size = grid.cells.len();

    let diff = get_puzzle_difficulty(&grid, target_difficulty >= 6).unwrap();
    let best_difficulty = Arc::new(Mutex::new((diff.star_count, diff.move_count)));
    let best_grid = Arc::new(Mutex::new(grid));
    let mut iterations = 0;

    let seen = Arc::new(Mutex::new(FxHashSet::<u128>::default()));

    while !queue.is_empty() {
        iterations += 1;
        {
            debug!(
                "queue len={} iter={} seen elements={}",
                queue.len(),
                iterations,
                seen.lock().unwrap().len()
            );
        }

        let mut pool = Vec::new();
        for _ in 0..2 * rayon::current_num_threads() {
            if let Some(item) = queue.pop() {
                pool.push(item);
            }
        }

        let mut next_candidates = pool
            .into_par_iter()
            .flat_map(|Task((star_count, move_count), mut rng, grid)| {
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
                        debug!("New best, difficulty = {}:\n{}", star_count, grid);

                        *best_difficulty = (star_count, move_count);
                        *best_grid.lock().unwrap() = grid.clone();
                    }
                }

                let mut candidates = grid
                    .iter_by_cell_pos_matching(|cell| matches!(cell, Cell::Solution(_)))
                    .into_iter()
                    .map(|cand| (cand, rng.clone()))
                    .collect::<Vec<_>>();
                candidates.shuffle(&mut rng);

                candidates
                    .into_par_iter()
                    .filter_map(|((x, y), rng)| {
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
                            Task((difficulty.star_count, difficulty.move_count), rng, grid)
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

pub fn generate_puzzle<Rand: Rng + Send + Clone>(
    size: usize,
    mut blocker_count: usize,
    mut blocker_num_count: usize,
    target_difficulty: usize,
    symmetric: bool,
    mut rng: &mut Rand,
) -> Option<(Grid, usize)> {
    let mut grid = Grid::parse(vec![format!("{}", "0".repeat(2 * size * size))]).unwrap();

    let mut blockers = grid
        .iter_by_cells()
        .into_iter()
        .filter(|((x, y), _)| {
            if symmetric {
                *x <= size / 2 && *y <= size / 2
            } else {
                true
            }
        })
        .map(|(pos, _)| pos)
        .collect::<Vec<_>>();
    blockers.shuffle(&mut rng);

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

    let mut blocker_cells = grid
        .iter_by_cell_pos_matching(|cell| matches!(cell, Cell::Black))
        .into_iter()
        .filter(|(x, y)| {
            if symmetric {
                *x <= size / 2 && *y <= size / 2
            } else {
                true
            }
        })
        .collect::<Vec<_>>();
    blocker_cells.shuffle(&mut rng);

    for (x, y) in blocker_cells.into_iter().take(blocker_num_count) {
        let n = rng.gen_range(1..=size);
        grid.cells[y][x] = Cell::Blocker(n as u8);
        if symmetric {
            let n2 = rng.gen_range(1..=size);
            grid.cells[size - y - 1][size - x - 1] = Cell::Blocker(n2 as u8);
        }
    }

    validate(&grid).ok()?;

    debug!("Attempting to generate\n{}", grid);

    let grid = recur(grid, rng)?;

    debug!("\nSolved grid:\n{}", grid);
    let final_grid = remove_numbers(grid, target_difficulty, symmetric, rng).unwrap();
    debug!("Calculating final difficulty");
    let difficulty = get_puzzle_difficulty(&final_grid, true).unwrap().star_count;
    debug!("Final difficulty: {}", difficulty);
    Some((final_grid, difficulty))
}

pub fn generator(
    size: usize,
    blocker_count: usize,
    blocker_num_count: usize,
    target_difficulty: usize,
    symmetric: bool,
) -> Grid {
    loop {
        let mut rng = rand_chacha::ChaCha8Rng::from_seed(thread_rng().gen());
        match generate_puzzle(
            size,
            blocker_count,
            blocker_num_count,
            target_difficulty,
            symmetric,
            &mut rng,
        ) {
            None => {}
            Some((grid, difficulty)) => {
                if difficulty == target_difficulty {
                    return grid;
                }
            }
        }
    }
}
