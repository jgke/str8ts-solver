use crate::grid::Grid;
use crate::solve_result::SolveType::*;
use crate::solve_result::ValidationError::*;
use crate::solve_result::{into_ty, SolveResults, SolveType, ValidationError, ValidationResult};
use crate::strategy::{Strategy, StrategyList};
use crate::strats;
use crate::validator::validate;

macro_rules! run_strat {
    ($list: ident, $strat: expr, $body: expr) => {
        if $list.has($strat) {
            if let Some(res) = $body? {
                return Ok(res);
            }
        }
    };
}

pub fn run_strat(grid: &mut Grid, strats: &StrategyList) -> Result<SolveResults, ValidationResult> {
    validate(grid)?;
    if grid.is_solved() {
        return Ok(PuzzleSolved.into());
    }

    let res = (|| {
        /* basic strats */
        run_strat!(strats, Strategy::UpdateImpossibles, strats::update_impossibles(grid));
        run_strat!(strats, Strategy::Singles, strats::singles(grid));
        run_strat!(strats, Strategy::Stranded, strats::stranded(grid));
        run_strat!(strats, Strategy::DefiniteMinMax, strats::definite_min_max(grid));
        run_strat!(strats, Strategy::RequiredRange, strats::required_range(grid));

        /* advanced strats */
        run_strat!(strats, Strategy::RequiredAndForbidden, strats::update_required_and_forbidden(grid));
        run_strat!(strats, Strategy::Setti, strats::setti(grid));
        run_strat!(strats, Strategy::RowColBrute, strats::row_col_brute(grid));
        run_strat!(strats, Strategy::YWing, strats::y_wing(grid));
        run_strat!(strats, Strategy::Fish, strats::fish(grid));

        run_strat!(strats, Strategy::Medusa, strats::medusa(grid));

        run_strat!(strats, Strategy::UniqueRequirement, strats::unique_requirement(grid));
        run_strat!(strats, Strategy::UniqueRequirementGuess, strats::unique_requirement_guess(grid));

        run_strat!(strats, Strategy::Guess, strats::guess(grid));

        run_strat!(strats, Strategy::EnumerateSolutions, strats::enumerate_solutions(grid));

        Err(OutOfStrats.into())
    })();
    strats::trivial(grid);
    validate(grid)?;
    res
}

pub fn run_fast_basic(grid: &mut Grid) -> Result<SolveType, ValidationResult> {
    run_strat(grid, &StrategyList::fast()).map(|t| t.ty)
}

pub fn solve_round(grid: &mut Grid, enable_guesses: bool) -> Result<SolveResults, ValidationResult> {
    if enable_guesses {
        run_strat(grid, &StrategyList::all())
    } else {
        run_strat(grid, &StrategyList::no_guesses())
    }
}

pub fn solve_basic(grid: &mut Grid) -> Result<SolveType, ValidationError> {
    loop {
        if into_ty(run_strat(grid, &StrategyList::basic()))? == PuzzleSolved {
            return Ok(PuzzleSolved);
        }
    }
}
