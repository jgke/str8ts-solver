use crate::difficulty::get_puzzle_difficulty;
use crate::grid::{Cell, Grid, Point};
use crate::solve_result::{into_ty, SolveType, ValidationError};
use crate::solver::run_strat;
use crate::strategy::StrategyList;
use crate::validator::validate;
use log::debug;
use rand::prelude::*;
use rand::{rng, SeedableRng};
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

pub fn fill_numbers<Rand: Rng + Send + Clone>(grid: Grid, rng: &mut Rand) -> Option<Grid> {
    let mut queue = BinaryHeap::new();
    let mut max_depth = 0;

    let order = {
        let mut order = grid
            .iter_by_indeterminates()
            .into_iter()
            .map(|(pos, set)| (pos, set.into_iter().collect::<Vec<_>>()))
            .collect::<Vec<_>>();
        order.shuffle(rng);
        for (_, set) in &mut order {
            set.shuffle(rng);
        }
        order
    };

    queue.push(Task(0, rng.clone(), grid));

    while let Some(Task(index, rng, grid)) = queue.pop() {
        if index > max_depth {
            max_depth = index;
            debug!("Reached {}/{} (queue={})", max_depth, order.len(), queue.len());
        }

        if index >= order.len() {
            assert!(grid.is_solved());
            return Some(grid);
        }

        let pos = order[index].0;
        let num_order = &order[index].1;

        let options = match grid.get_cell(pos) {
            Cell::Indeterminate(set) => Some(
                num_order
                    .iter()
                    .copied()
                    .filter(|&num| set.contains(num))
                    .map(|num| (num, rng.clone()))
                    .collect::<Vec<_>>(),
            )
            .filter(|set| !set.is_empty()),
            _ => None,
        };

        if let Some(options) = options {
            let vec = options
                .into_par_iter()
                .filter_map(|(num, rng)| {
                    let mut new_grid = grid.clone();
                    new_grid.set_cell(pos, Cell::Requirement(num));
                    loop {
                        match into_ty(run_strat(&mut new_grid, &StrategyList::no_guesses())) {
                            Err(ValidationError::OutOfStrats) | Ok(SolveType::PuzzleSolved) => break,
                            Ok(_) => {}
                            Err(_) => return None,
                        }
                    }
                    Some(Task(index + 1, rng, new_grid))
                })
                .collect::<Vec<_>>();
            queue.extend(vec);
        } else {
            queue.push(Task(index + 1, rng, grid));
        }
    }
    None
}

pub fn generate_solved_grid<Rand: Rng + Send + Clone>(
    size: usize,
    blocker_count: usize,
    blocker_num_count: usize,
    symmetric: bool,
    rng: &mut Rand,
) -> Option<Grid> {
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
    blockers.shuffle(rng);

    for (x, y) in blockers
        .into_iter()
        .take(if symmetric { blocker_count / 2 } else { blocker_count })
    {
        grid.set_cell((x, y), Cell::Black);
        if symmetric {
            grid.set_cell((size - x - 1, size - y - 1), Cell::Black);
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
    blocker_cells.shuffle(rng);

    for (x, y) in blocker_cells.into_iter().take(if symmetric {
        blocker_num_count / 2
    } else {
        blocker_num_count
    }) {
        let n = rng.random_range(1..=size);
        grid.set_cell((x, y), Cell::Blocker(n as u8));
        if symmetric {
            let n2 = rng.random_range(1..=size);
            grid.set_cell((size - x - 1, size - y - 1), Cell::Blocker(n2 as u8));
        }
    }

    validate(&grid).ok()?;

    debug!("Attempting to generate\n{}", grid);

    fill_numbers(grid, rng)
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
    let mut queue: BinaryHeap<Task<Point, Rand>> = BinaryHeap::new();
    queue.push(Task((0, 0), rng.clone(), grid.clone()));
    let size = grid.y;
    let strats = StrategyList::for_difficulty(target_difficulty);

    let diff = get_puzzle_difficulty(&grid, &strats).unwrap();
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
                    .filter_map(|((x, y), mut rng)| {
                        let mut grid = grid.clone();
                        grid.set_cell((x, y), Cell::Indeterminate((1..=size as u8).collect()));
                        if symmetric {
                            grid.set_cell(
                                (size - x - 1, size - y - 1),
                                Cell::Indeterminate((1..=size as u8).collect()),
                            );
                        } else {
                            let nx = rng.random_range(0..size);
                            let ny = rng.random_range(0..size);
                            if let Cell::Solution(_) = grid.get_cell((nx, ny)) {
                                grid.set_cell(
                                    (size - x - 1, size - y - 1),
                                    Cell::Indeterminate((1..=size as u8).collect()),
                                );
                            }
                        }
                        {
                            let grid_hash = get_grid_hash(&grid);
                            let mut seen = seen.lock().unwrap();
                            if seen.contains(&grid_hash) {
                                return None;
                            }
                            seen.insert(grid_hash);
                        }
                        get_puzzle_difficulty(&grid, &strats)
                            .map(|difficulty| Task((difficulty.star_count, difficulty.move_count), rng, grid))
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

fn generate_puzzle<Rand: Rng + Send + Clone>(
    size: usize,
    blocker_count: usize,
    blocker_num_count: usize,
    target_difficulty: usize,
    symmetric: bool,
    rng: &mut Rand,
) -> Option<(Grid, usize)> {
    let grid = generate_solved_grid(size, blocker_count, blocker_num_count, symmetric, rng)?;

    debug!("\nSolved grid:\n{}", grid);
    let final_grid = remove_numbers(grid, target_difficulty, symmetric, rng).unwrap();
    debug!("Calculating final difficulty");
    let difficulty = get_puzzle_difficulty(&final_grid, &StrategyList::all())
        .unwrap()
        .star_count;
    debug!("Final difficulty: {}", difficulty);
    Some((final_grid, difficulty))
}

pub fn generator_loop(
    size: usize,
    blocker_count: usize,
    blocker_num_count: usize,
    mut target_difficulty: usize,
    symmetric: bool,
    first_seed: u64,
) -> Grid {
    let mut seed = first_seed;
    if target_difficulty == 3 {
        target_difficulty = 4;
    } else if target_difficulty > 7 {
        target_difficulty = 7;
    }
    loop {
        let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(seed);
        match generate_puzzle(size, blocker_count, blocker_num_count, target_difficulty, symmetric, &mut rng) {
            None => {}
            Some((grid, difficulty)) => {
                if difficulty == target_difficulty {
                    return grid;
                }
            }
        }
        seed = rng.next_u64();
    }
}

pub fn generator(
    size: usize,
    blocker_count: usize,
    blocker_num_count: usize,
    target_difficulty: usize,
    symmetric: bool,
) -> Grid {
    let mut rng = rand_chacha::ChaCha8Rng::from_seed(rng().random());
    generator_loop(size, blocker_count, blocker_num_count, target_difficulty, symmetric, rng.next_u64())
}
