use crate::grid::Grid;
use crate::solver::{solve_round, SolveResults};

#[derive(Debug, Clone)]
pub struct Difficulty {
    pub star_count: usize,
    pub move_count: usize,
    pub basic_reductions: bool,
    pub min_max_reductions: bool,
    pub cross_compartment_ranges: bool,
    pub maintain_reqs_and_blocks: bool,
    pub sets: bool,
    pub setti: bool,
    pub y_wing: bool,
    pub x_wing: bool,
    pub swordfish: bool,
    pub medusa: bool,
    pub n_fish: usize,
    pub unique_requirement_single: bool,
    pub unique_requirement_count: usize,
    pub short_chain_count: usize,
    pub long_chain_count: usize,
}

pub fn puzzle_difficulty(history: &[&SolveResults]) -> Difficulty {
    let move_count = history.len();

    let mut star_count = history
        .iter()
        .map(|res| res.difficulty())
        .max()
        .unwrap_or(0);

    if (star_count == 1 || star_count == 3) && move_count > 30 {
        star_count += 1;
    }

    Difficulty {
        star_count,
        move_count,
        basic_reductions: history.len() > 1,
        min_max_reductions: history
            .iter()
            .any(|e| matches!(e, SolveResults::DefiniteMinMax)),
        cross_compartment_ranges: history
            .iter()
            .any(|e| matches!(e, SolveResults::RequiredRange)),
        sets: history.iter().any(|e| matches!(e, SolveResults::Sets(_))),
        maintain_reqs_and_blocks: history
            .iter()
            .any(|e| matches!(e, SolveResults::RequiredAndForbidden)),
        setti: history.iter().any(|e| matches!(e, SolveResults::Setti(_))),
        y_wing: history
            .iter()
            .any(|e| matches!(e, SolveResults::YWing(_, _))),
        x_wing: history.iter().any(|e| matches!(e, SolveResults::Fish(2))),
        swordfish: history.iter().any(|e| matches!(e, SolveResults::Fish(3))),
        medusa: history.iter().any(|e| matches!(e, SolveResults::Fish(4))),
        n_fish: history
            .iter()
            .map(|e| if let SolveResults::Fish(n) = e { *n } else { 0 })
            .max()
            .unwrap_or(0),
        unique_requirement_single: history
            .iter()
            .any(|e| matches!(e, SolveResults::SimpleUniqueRequirement(..))),
        unique_requirement_count: history
            .iter()
            .filter(|e| matches!(e, SolveResults::UniqueRequirement(..)))
            .count(),
        short_chain_count: history
            .iter()
            .filter(|e| matches!(e, SolveResults::Chain(_, _, steps, _) if steps.len() < 8))
            .count(),
        long_chain_count: history
            .iter()
            .filter(|e| matches!(e, SolveResults::Chain(_, _, steps, _) if steps.len() >= 8))
            .count(),
    }
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
