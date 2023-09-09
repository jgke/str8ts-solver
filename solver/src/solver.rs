use crate::grid::Grid;
use crate::solver::SolveResults::*;
use crate::solver::ValidationResult::*;
use crate::strats;
use crate::validator::validate;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SolveResults {
    UpdateImpossibles,
    Singles,
    Stranded,
    DefiniteMinMax,
    RequiredRange,
    Sets(usize),
    RequiredAndForbidden,
    RowColBrute,
    Setti,
    Fish(usize),
    StartChain((usize, usize), u8),
    Chain((usize, usize), u8, Rc<Vec<(Grid, SolveResults)>>, Grid),
    EndChain(ValidationResult),
    PuzzleSolved,
    OutOfBasicStrats,
}

impl SolveResults {
    pub fn difficulty(&self) -> usize {
        match self {
            UpdateImpossibles => 1,
            Stranded => 1,
            DefiniteMinMax => 1,
            Singles => 2,
            RequiredRange => 3,
            Sets(_) => 3,
            RequiredAndForbidden => 5,
            RowColBrute => 5,
            Setti => 5,
            Fish(2) | Fish(3) => 5,
            Fish(_) => 6,
            StartChain(_, _) => 1,
            Chain(_, _, steps, _) if steps.len() < 8 => 6,
            Chain(_, _, _, _) => 7,
            EndChain(_) => 1,
            PuzzleSolved => 1,
            OutOfBasicStrats => 0,
        }
    }
}

impl Display for SolveResults {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            UpdateImpossibles => write!(f, "Remove trivially impossible numbers"),
            Stranded => write!(f, "Remove stranded numbers"),
            DefiniteMinMax => write!(f, "Remove unreachable numbers from compartments"),
            Singles => write!(f, "Find hidden singles"),
            RequiredRange => write!(
                f,
                "Remove numbers from other compartments if they are required in others"
            ),
            Sets(n) => write!(f, "Find out sets of {} numbers", n),
            RequiredAndForbidden => write!(f, "List required numbers and blocked numbers"),
            RowColBrute => write!(
                f,
                "Think very hard about possible combinations in rows and columns"
            ),
            Setti => write!(f, "Calculate settis"),
            Fish(2) => write!(f, "Calculate a X-wing"),
            Fish(3) => write!(f, "Calculate a Swordfish"),
            Fish(n) => write!(f, "Calculate a {}-fish", n),
            StartChain((x, y), n) => write!(f, "Start chain with ({}, {}) = {}", x + 1, y + 1, n),
            Chain((x, y), n, steps, _) => write!(
                f,
                "({}, {}) cannot be {}, as it causes a conflict in {} steps",
                x + 1,
                y + 1,
                n,
                steps.len(),
            ),
            EndChain(end) => write!(f, "{}", end),
            PuzzleSolved => write!(f, "Puzzle solved"),
            OutOfBasicStrats => write!(f, "Out of basic strats"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ValidationResult {
    EmptyCell {
        pos: (usize, usize),
    },
    Conflict {
        pos1: (usize, usize),
        pos2: (usize, usize),
        val: u8,
    },
    Sequence {
        vertical: bool,
        top_left: (usize, usize),
        range: (u8, u8),
        missing: u8,
    },
    SequenceTooLarge {
        vertical: bool,
        top_left: (usize, usize),
        contains: (u8, u8),
        max_ranges: ((u8, u8), (u8, u8)),
    },
    OutOfStrats,
}

impl Display for ValidationResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EmptyCell { pos: (x, y) } => write!(
                f,
                "Cell scan: Cell ({}, {}) ran out of possible options",
                x + 1,
                y + 1
            ),
            Conflict { pos1, pos2, val } => write!(
                f,
                "Cells ({}, {}) and ({}, {}) both contain {}",
                pos1.0 + 1,
                pos1.1 + 1,
                pos2.0 + 1,
                pos2.1 + 1,
                val
            ),
            Sequence { vertical, missing, range: (min, max), top_left: (x, y) } =>
                write!(f, "Compartment scan: {} compartment starting from ({}, {}) contains numbers {} and {}, but it doesn't contain {} making it non-contiguous",
                if *vertical { "Vertical" } else { "Horizontal" }, x+1, y+1, min, max, missing),
            SequenceTooLarge { vertical, max_ranges: ((min_a, max_a), (min_b, max_b)), top_left: (x, y), contains: (a, b) } =>
                write!(f, "Compartment scan: {} compartment starting from ({}, {}) contains numbers {} and {}, but it's too small for them as it can either contain {} to {} or {} to {}",
                       if *vertical { "Vertical" } else { "Horizontal" }, x+1, y+1, a, b, min_a, max_a, min_b, max_b),
            OutOfStrats => write!(f, "Ran out of strategies!"),
        }
    }
}

pub fn run_fast_basic(grid: &mut Grid) -> SolveResults {
    let res = {
        if strats::update_impossibles(grid) {
            UpdateImpossibles
        } else if strats::singles(grid) {
            Singles
        } else if strats::stranded(grid) {
            Stranded
        } else if strats::definite_min_max(grid) {
            DefiniteMinMax
        } else if strats::required_range(grid) {
            RequiredRange
        } else {
            OutOfBasicStrats
        }
    };

    strats::trivial(grid);
    res
}

pub fn run_basic(grid: &mut Grid) -> SolveResults {
    let res = {
        if strats::update_impossibles(grid) {
            UpdateImpossibles
        } else if strats::singles(grid) {
            Singles
        } else if strats::stranded(grid) {
            Stranded
        } else if strats::definite_min_max(grid) {
            DefiniteMinMax
        } else if strats::required_range(grid) {
            RequiredRange
        } else if let Some(n) = strats::sets(grid) {
            Sets(n)
        } else {
            OutOfBasicStrats
        }
    };

    strats::trivial(grid);
    res
}

pub fn solve_round(grid: &mut Grid, enable_chains: bool) -> Result<SolveResults, ValidationResult> {
    validate(grid)?;
    if grid.is_solved() {
        return Ok(PuzzleSolved);
    }
    match run_basic(grid) {
        OutOfBasicStrats => {
            validate(grid)?;
            let res = {
                if strats::update_required_and_forbidden(grid) {
                    Ok(RequiredAndForbidden)
                } else if strats::setti(grid) {
                    Ok(Setti)
                } else if strats::row_col_brute(grid) {
                    Ok(RowColBrute)
                } else if let Some(n) = strats::fish(grid) {
                    Ok(Fish(n))
                } else if enable_chains {
                    if let Some(((x, y), n, steps, error_grid)) = strats::chain(grid) {
                        Ok(Chain((x, y), n, Rc::new(steps), error_grid))
                    } else {
                        return Err(OutOfStrats);
                    }
                } else {
                    return Err(OutOfStrats);
                }
            };
            strats::trivial(grid);
            validate(grid)?;
            res
        }
        otherwise => {
            validate(grid)?;
            Ok(otherwise)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::*;

    #[test]
    fn test_invalid_row() {
        let grid = g("
####
#44#
####
####
");
        assert_eq!(
            validate(&grid),
            Err(Conflict {
                pos1: (2, 2),
                pos2: (3, 2),
                val: 4
            })
        );
    }

    #[test]
    fn test_invalid_col() {
        let grid = g("
####
#4##
#4##
####
");
        assert_eq!(
            validate(&grid),
            Err(Conflict {
                pos1: (2, 2),
                pos2: (2, 3),
                val: 4
            })
        );
    }

    #[test]
    fn test_invalid_sequence() {
        let grid = g("
####
#124
####
####
");
        assert_eq!(
            validate(&grid),
            Err(Sequence {
                range: (1, 4),
                vertical: false,
                missing: 3,
                top_left: (1, 1)
            })
        );
    }
}
