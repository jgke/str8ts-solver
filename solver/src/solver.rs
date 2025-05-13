use crate::bitset::BitSet;
use crate::grid::{Grid, Point};
use crate::solver::SolveType::*;
use crate::solver::ValidationError::*;
use crate::strats;
use crate::strats::{enumerate_solutions, UrResult};
use crate::validator::validate;
use itertools::intersperse;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct SolveMetadata {
    pub colors: Vec<Vec<(Point, u8)>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SolveType {
    UpdateImpossibles,
    Singles,
    Stranded,
    DefiniteMinMax,
    RequiredRange,
    Sets(usize),
    RequiredAndForbidden,
    RowColBrute,
    Setti(BitSet),
    YWing(Point, u8),
    Fish(usize),
    Medusa,
    UniqueRequirement(UrResult),
    StartGuess(Point, u8),
    GuessStep(Point, u8, Rc<Vec<(Grid, SolveResults)>>, Grid),
    EndGuess(ValidationResult),
    PuzzleSolved,
    EnumerateSolutions,
    OutOfBasicStrats,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SolveResults {
    pub ty: SolveType,
    pub meta: SolveMetadata,
}

impl From<SolveType> for SolveResults {
    fn from(ty: SolveType) -> Self {
        let meta = SolveMetadata::default();
        SolveResults { ty, meta }
    }
}

impl SolveType {
    pub fn difficulty(&self) -> usize {
        match self {
            UpdateImpossibles => 1,
            Stranded => 1,
            DefiniteMinMax => 2,
            Singles => 2,
            RequiredRange => 4,
            Sets(_) => 4,
            RequiredAndForbidden => 5,
            RowColBrute => 5,
            Setti(_) => 5,
            YWing(_, _) => 5,
            Fish(2) | Fish(3) => 5,
            Fish(_) => 6,
            Medusa => 6,
            UniqueRequirement(..) => 6,
            StartGuess(_, _) => 7,
            GuessStep(_, _, _, _) => 7,
            EndGuess(_) => 7,
            PuzzleSolved => 1,
            EnumerateSolutions => 7,
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
        match &self.ty {
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
            YWing((x, y), n) => write!(f, "Y-Wing causes ({}, {}) to not be {}", x + 1, y + 1, n),
            Fish(2) => write!(f, "Calculate a X-wing"),
            Fish(3) => write!(f, "Calculate a Swordfish"),
            Fish(n) => write!(f, "Calculate a {}-fish", n),
            Medusa => write!(f, "Calculate a 3D Medusa"),
            UniqueRequirement(UrResult::SingleUnique((x, y), n)) => {
                write!(
                    f,
                    "({}, {}) must be {}, as other solutions would be ambiguous",
                    x + 1,
                    y + 1,
                    n,
                )
            }
            UniqueRequirement(UrResult::IntraCompartmentUnique((x, y), n)) => {
                write!(
                    f,
                    "({}, {}) cannot be {}, as it would cause ambiguous solutions",
                    x + 1,
                    y + 1,
                    n,
                )
            }
            UniqueRequirement(UrResult::ClosedSetCompartment(list, n)) => {
                write!(
                    f,
                    "The cells {:?} must contain {} or the puzzle becomes ambiguous",
                    list.iter().map(|(x, y)| (x + 1, y + 1)).collect::<Vec<_>>(),
                    n,
                )
            }
            UniqueRequirement(UrResult::SingleCellWouldBecomeFree((x, y), n)) => {
                write!(
                    f,
                    "({}, {}) cannot be {}, as it would cause ambiguous solutions",
                    x + 1,
                    y + 1,
                    n,
                )
            }
            UniqueRequirement(UrResult::UrSetti(list, vertical, n)) => {
                write!(
                    f,
                    "The {} containing points {:?} must contain {}, or the puzzle becomes ambiguous",
                    if *vertical { "columns" } else { "rows" },
                    list.iter().map(|(x, y)| (x + 1, y + 1)).collect::<Vec<_>>(),
                    n,
                )
            }
            UniqueRequirement(UrResult::SolutionCausesClosedSets((x, y), n)) => {
                write!(
                    f,
                    "Setting ({}, {}) to {} creates closed sets, causing puzzle to become ambiguous",
                    x + 1, y + 1, n,
                )
            }
            StartGuess((x, y), n) => write!(f, "Start guess with ({}, {}) = {}", x + 1, y + 1, n),
            GuessStep((x, y), n, steps, _) => write!(
                f,
                "({}, {}) cannot be {}, as it causes a conflict in {} steps",
                x + 1,
                y + 1,
                n,
                steps.len(),
            ),
            EndGuess(end) => write!(f, "{}", end),
            PuzzleSolved => write!(f, "Puzzle solved"),
            EnumerateSolutions => write!(f, "Enumerate all possible solutions"),
            OutOfBasicStrats => write!(f, "Out of basic strats"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ValidationError {
    EmptyCell {
        pos: Point,
    },
    Conflict {
        pos1: Point,
        pos2: Point,
        val: u8,
    },
    Sequence {
        vertical: bool,
        top_left: Point,
        range: (u8, u8),
        missing: u8,
    },
    SequenceTooLarge {
        vertical: bool,
        top_left: Point,
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
        cells: Vec<Point>,
    },
    NoSolutions,
    OutOfStrats,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValidationResult {
    pub ty: ValidationError,
    pub meta: SolveMetadata,
}

impl From<ValidationError> for ValidationResult {
    fn from(ty: ValidationError) -> Self {
        let meta = SolveMetadata::default();
        ValidationResult { ty, meta }
    }
}

pub fn into_ty(res: Result<SolveResults, ValidationResult>) -> Result<SolveType, ValidationError> {
    match res {
        Ok(SolveResults { ty, meta: _ }) => Ok(ty),
        Err(ValidationResult { ty, meta: _ }) => Err(ty),
    }
}

impl Display for ValidationResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.ty {
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
            NoSolutions => write!(f, "Exhaustive search proves grid has no solutions"),
            OutOfStrats => write!(f, "Ran out of strategies!"),
        }
    }
}

pub fn run_fast_basic(grid: &mut Grid) -> Result<SolveType, ValidationResult> {
    let res: SolveType = {
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
            UpdateImpossibles.into()
        } else if strats::singles(grid)? {
            Singles.into()
        } else if strats::stranded(grid)? {
            Stranded.into()
        } else if strats::definite_min_max(grid)? {
            DefiniteMinMax.into()
        } else if strats::required_range(grid)? {
            RequiredRange.into()
        } else if grid.has_requirements() && strats::update_required_and_forbidden(grid)? {
            RequiredAndForbidden.into()
        } else if let Some(res) = strats::sets(grid)? {
            res
        } else {
            OutOfBasicStrats.into()
        }
    };

    strats::trivial(grid);

    Ok(res)
}

pub fn run_advanced(grid: &mut Grid) -> Result<Option<SolveResults>, ValidationResult> {
    Ok(Some(if strats::update_required_and_forbidden(grid)? {
        RequiredAndForbidden.into()
    } else if let Some(set) = strats::setti(grid) {
        Setti(set).into()
    } else if strats::row_col_brute(grid)? {
        RowColBrute.into()
    } else if let Some(res) = strats::y_wing(grid)? {
        res
    } else if let Some(res) = strats::fish(grid)? {
        res
    } else {
        return Ok(None);
    }))
}

pub fn run_medusa(
    grid: &mut Grid,
    enable_guesses: bool,
) -> Result<Option<SolveResults>, ValidationResult> {
    if !enable_guesses {
        return Ok(None);
    }
    Ok(if let Some((left, right)) = strats::medusa(grid)? {
        let meta = SolveMetadata {
            colors: vec![left, right],
        };
        Some(SolveResults { ty: Medusa, meta })
    } else {
        None
    })
}

pub fn run_unique(
    grid: &mut Grid,
    enable_guesses: bool,
) -> Result<Option<SolveResults>, ValidationResult> {
    strats::unique_requirement(grid, enable_guesses)
}

pub fn run_guess(
    grid: &mut Grid,
    enable_guesses: bool,
) -> Result<Option<SolveResults>, ValidationResult> {
    if !enable_guesses {
        return Ok(None);
    }
    Ok(strats::guess(grid)?.map(
        |crate::strats::GuessSolveResult(((x, y), n, steps, error_grid))| {
            GuessStep((x, y), n, Rc::new(steps), error_grid).into()
        },
    ))
}

pub fn run_enumerate(
    grid: &mut Grid,
    enable_guesses: bool,
) -> Result<Option<SolveResults>, ValidationResult> {
    if !enable_guesses {
        return Ok(None);
    }
    enumerate_solutions(grid)
}

pub fn solve_round(
    grid: &mut Grid,
    enable_guesses: bool,
) -> Result<SolveResults, ValidationResult> {
    validate(grid)?;
    if grid.is_solved() {
        return Ok(PuzzleSolved.into());
    }
    match run_basic(grid)? {
        SolveResults {
            ty: OutOfBasicStrats,
            meta: _,
        } => {
            validate(grid)?;
            let res: Result<SolveResults, ValidationResult> = if let Some(res) = run_advanced(grid)?
            {
                Ok(res)
            } else if let Some(res) = run_medusa(grid, enable_guesses)? {
                Ok(res)
            } else if let Some(res) = run_unique(grid, enable_guesses)? {
                Ok(res)
            } else if let Some(res) = run_guess(grid, enable_guesses)? {
                Ok(res)
            } else if let Some(res) = run_enumerate(grid, enable_guesses)? {
                Ok(res)
            } else {
                Err(OutOfStrats.into())
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

pub fn solve_basic(grid: &mut Grid) -> Result<SolveType, ValidationResult> {
    while run_basic(grid)?.ty != OutOfBasicStrats {}
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
            Err(ValidationResult {
                ty: Conflict {
                    pos1: (2, 2),
                    pos2: (3, 2),
                    val: 4
                },
                meta: SolveMetadata::default()
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
            Err(ValidationResult {
                ty: Conflict {
                    pos1: (2, 2),
                    pos2: (2, 3),
                    val: 4
                },
                meta: SolveMetadata::default()
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
            Err(ValidationResult {
                ty: Sequence {
                    range: (1, 4),
                    vertical: false,
                    missing: 3,
                    top_left: (1, 1)
                },
                meta: SolveMetadata::default()
            })
        );
    }
}
