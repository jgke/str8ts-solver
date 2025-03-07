use crate::bitset::BitSet;
use crate::grid::Grid;
use crate::solver::SolveResults::*;
use crate::solver::ValidationResult::*;
use crate::strats;
use crate::validator::validate;
use itertools::intersperse;
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
    Setti(BitSet),
    Fish(usize),
    UniqueRequirementSingleCell((usize, usize), bool, u8),
    UniqueRequirement((usize, usize), u8, Rc<Vec<(Grid, SolveResults)>>, Grid),
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
            Setti(_) => 5,
            Fish(2) | Fish(3) => 5,
            Fish(_) => 6,
            UniqueRequirementSingleCell(..) => 6,
            UniqueRequirement(..) => 7,
            StartChain(_, _) => 1,
            Chain(_, _, steps, _) if steps.len() < 8 => 6,
            Chain(_, _, _, _) => 7,
            EndChain(_) => 1,
            PuzzleSolved => 1,
            OutOfBasicStrats => 0,
        }
    }
}

fn english_list<T: ToString>(list: &[T]) -> String {
    match list.len() {
        0 => "".to_string(),
        1 => list[0].to_string(),
        _ => {
            let (last, rest) = list.split_last().unwrap();
            format!(
                "{} and {}",
                intersperse(rest.iter().map(|s| s.to_string()), ", ".to_string())
                    .collect::<String>(),
                last.to_string()
            )
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
            Setti(set) => write!(
                f,
                "Calculate settis on {}",
                english_list(&set.into_iter().collect::<Vec<_>>())
            ),
            Fish(2) => write!(f, "Calculate a X-wing"),
            Fish(3) => write!(f, "Calculate a Swordfish"),
            Fish(n) => write!(f, "Calculate a {}-fish", n),
            UniqueRequirementSingleCell((x, y), b, n) => {
                if *b {
                    write!(
                        f,
                        "({}, {}) must be {}, as other solutions would be ambiguous",
                        x + 1,
                        y + 1,
                        n,
                    )
                } else {
                    write!(
                        f,
                        "({}, {}) cannot be {}, as it would cause ambiguous solutions",
                        x + 1,
                        y + 1,
                        n,
                    )
                }
            }
            UniqueRequirement((x, y), n, steps, _) => {
                write!(
                    f,
                    "({}, {}) cannot be {}, as it causes a unique requirement conflict in {} steps",
                    x + 1,
                    y + 1,
                    n,
                    steps.len(),
                )
            }
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
    RequirementBlockerConflict {
        vertical: bool,
        index: usize,
        number: u8,
    },
    RequiredNumberMissing {
        vertical: bool,
        index: usize,
        number: u8,
    },
    BlockedNumberPresent {
        vertical: bool,
        index: usize,
        number: u8,
    },
    Ambiguous {
        cells: Vec<(usize, usize)>,
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
            RequirementBlockerConflict { vertical, index, number } =>
                write!(f, "The number {} is included both in required and blocked numbers for {} {}",
                       number, if *vertical {"column"} else {"row"}, index + 1
                       ),
            RequiredNumberMissing { vertical, index, number } =>
                write!(f, "The number {} is required in {} {} but not present in any container",
                       number, if *vertical {"column"} else {"row"}, index + 1
                       ),
            BlockedNumberPresent { vertical, index, number } =>
                write!(f, "The number {} is forbidden in {} {} but is a solution or a requirement",
                       number, if *vertical {"column"} else {"row"}, index + 1
                       ),
            Ambiguous { .. } => write!(f, "Grid is ambiguous, and cannot be solved"),
            OutOfStrats => write!(f, "Ran out of strategies!"),
        }
    }
}

pub fn run_fast_basic(grid: &mut Grid) -> Result<SolveResults, ValidationResult> {
    let res = {
        if strats::update_impossibles(grid)? {
            UpdateImpossibles
        } else if strats::singles(grid)? {
            Singles
        } else if strats::stranded(grid)? {
            Stranded
        } else if strats::definite_min_max(grid)? {
            DefiniteMinMax
        } else if strats::required_range(grid)? {
            RequiredRange
        } else {
            OutOfBasicStrats
        }
    };

    strats::trivial(grid);
    Ok(res)
}

pub fn run_basic(grid: &mut Grid) -> Result<SolveResults, ValidationResult> {
    let res = {
        if strats::update_impossibles(grid)? {
            UpdateImpossibles
        } else if strats::singles(grid)? {
            Singles
        } else if strats::stranded(grid)? {
            Stranded
        } else if strats::definite_min_max(grid)? {
            DefiniteMinMax
        } else if strats::required_range(grid)? {
            RequiredRange
        } else if let Some(n) = strats::sets(grid)? {
            Sets(n)
        } else {
            OutOfBasicStrats
        }
    };

    strats::trivial(grid);

    Ok(res)
}

pub fn run_advanced(grid: &mut Grid) -> Result<Option<SolveResults>, ValidationResult> {
    Ok(Some(if strats::update_required_and_forbidden(grid)? {
        RequiredAndForbidden
    } else if let Some(set) = strats::setti(grid) {
        Setti(set)
    } else if strats::row_col_brute(grid)? {
        RowColBrute
    } else if let Some(n) = strats::fish(grid)? {
        Fish(n)
    } else {
        return Ok(None);
    }))
}

pub fn run_unique(grid: &mut Grid) -> Result<Option<SolveResults>, ValidationResult> {
    Ok(
        if let Some((pos, b, n)) = strats::unique_requirement(grid)? {
            Some(UniqueRequirementSingleCell(pos, b, n))
        } else {
            None
        },
    )
}

pub fn run_chain(
    grid: &mut Grid,
    enable_chains: bool,
) -> Result<Option<SolveResults>, ValidationResult> {
    if !enable_chains {
        return Ok(None);
    }
    Ok(match strats::chain(grid)? {
        Some(crate::strats::ChainSolveResult::NotUnique(((x, y), n, steps, error_grid))) => {
            Some(UniqueRequirement((x, y), n, Rc::new(steps), error_grid))
        }
        Some(crate::strats::ChainSolveResult::Error(((x, y), n, steps, error_grid))) => {
            Some(Chain((x, y), n, Rc::new(steps), error_grid))
        }
        None => None,
    })
}

pub fn solve_round(grid: &mut Grid, enable_chains: bool) -> Result<SolveResults, ValidationResult> {
    validate(grid)?;
    if grid.is_solved() {
        return Ok(PuzzleSolved);
    }
    match run_basic(grid)? {
        OutOfBasicStrats => {
            validate(grid)?;
            let res = if let Some(res) = run_advanced(grid)? {
                Ok(res)
            } else if let Some(res) = run_unique(grid)? {
                Ok(res)
            } else if let Some(res) = run_chain(grid, enable_chains)? {
                Ok(res)
            } else {
                Err(OutOfStrats)
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

pub fn solve_basic(grid: &mut Grid) -> Result<SolveResults, ValidationResult> {
    while run_basic(grid)? != OutOfBasicStrats {}
    Ok(OutOfBasicStrats)
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
