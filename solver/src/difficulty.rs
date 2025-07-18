use crate::grid::Grid;
use crate::solve_result::{into_ty, SolveType};
use crate::solver::run_strat;
use crate::strategy::{Strategy, StrategyList};

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
    pub n_fish: usize,
    pub medusa: bool,
    pub unique_requirement: bool,
    pub short_guess_count: usize,
    pub long_guess_count: usize,
}

pub fn puzzle_difficulty(history: &[&SolveType]) -> Difficulty {
    let move_count = history.len();

    let star_count = history
        .iter()
        .map(|&res| Strategy::from(res.clone()).difficulty())
        .max()
        .unwrap_or(0);

    Difficulty {
        star_count,
        move_count,
        basic_reductions: history.len() > 1,
        min_max_reductions: history.iter().any(|e| matches!(e, SolveType::DefiniteMinMax)),
        cross_compartment_ranges: history.iter().any(|e| matches!(e, SolveType::RequiredRange)),
        sets: history.iter().any(|e| matches!(e, SolveType::Sets(_))),
        maintain_reqs_and_blocks: history.iter().any(|e| matches!(e, SolveType::RequiredAndForbidden)),
        setti: history.iter().any(|e| matches!(e, SolveType::Setti(_))),
        y_wing: history.iter().any(|e| matches!(e, SolveType::YWing(_, _))),
        x_wing: history.iter().any(|e| matches!(e, SolveType::Fish(2))),
        swordfish: history.iter().any(|e| matches!(e, SolveType::Fish(3))),
        n_fish: history
            .iter()
            .map(|e| if let SolveType::Fish(n) = e { *n } else { 0 })
            .max()
            .unwrap_or(0),
        medusa: history.iter().any(|e| matches!(e, SolveType::Medusa)),
        unique_requirement: history.iter().any(|e| matches!(e, SolveType::UniqueRequirement(..))),
        short_guess_count: history
            .iter()
            .filter(|e| matches!(e, SolveType::GuessStep(_, _, steps, _) if steps.len() < 8))
            .count(),
        long_guess_count: history
            .iter()
            .filter(|e| matches!(e, SolveType::GuessStep(_, _, steps, _) if steps.len() >= 8))
            .count(),
    }
}

pub fn get_puzzle_difficulty(grid: &Grid, strats: &StrategyList) -> Option<Difficulty> {
    let solution = {
        let mut grid = grid.clone();
        let mut history = Vec::new();
        loop {
            match into_ty(run_strat(&mut grid, strats)) {
                Ok(SolveType::PuzzleSolved) => {
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
