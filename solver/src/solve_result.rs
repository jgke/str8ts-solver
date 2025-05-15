use crate::bitset::BitSet;
use crate::grid::{Grid, Point};
use crate::solve_result::SolveType::*;
use crate::solve_result::ValidationError::*;
use crate::strategy::Strategy;
use crate::strats::UrResult;
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

impl From<SolveType> for Strategy {
    fn from(ty: SolveType) -> Self {
        match ty {
            UpdateImpossibles => Strategy::UpdateImpossibles,
            Singles => Strategy::Singles,
            Stranded => Strategy::Stranded,
            DefiniteMinMax => Strategy::DefiniteMinMax,
            RequiredRange => Strategy::RequiredRange,
            Sets(_) => Strategy::Sets,
            RequiredAndForbidden => Strategy::RequiredAndForbidden,
            RowColBrute => Strategy::RowColBrute,
            Setti(_) => Strategy::Setti,
            YWing(_, _) => Strategy::YWing,
            Fish(_) => Strategy::Fish,
            Medusa => Strategy::Medusa,
            UniqueRequirement(_) => Strategy::UniqueRequirement,
            StartGuess(_, _) => Strategy::Guess,
            GuessStep(_, _, _, _) => Strategy::Guess,
            EndGuess(_) => Strategy::Guess,
            EnumerateSolutions => Strategy::EnumerateSolutions,
            // XXX
            PuzzleSolved => Strategy::UpdateImpossibles,
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
                intersperse(rest.iter().map(|s| s.to_string()), ", ".to_string()).collect::<String>(),
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
            RequiredRange => {
                write!(f, "Remove numbers from other compartments if they are required in others")
            }
            Sets(n) => write!(f, "Find out sets of {} numbers", n),
            RequiredAndForbidden => write!(f, "List required numbers and blocked numbers"),
            RowColBrute => {
                write!(f, "Think very hard about possible combinations in rows and columns")
            }
            Setti(set) => write!(f, "Calculate settis on {}", english_list(&set.into_iter().collect::<Vec<_>>())),
            YWing((x, y), n) => write!(f, "Y-Wing causes ({}, {}) to not be {}", x + 1, y + 1, n),
            Fish(2) => write!(f, "Calculate a X-wing"),
            Fish(3) => write!(f, "Calculate a Swordfish"),
            Fish(n) => write!(f, "Calculate a {}-fish", n),
            Medusa => write!(f, "Calculate a 3D Medusa"),
            UniqueRequirement(UrResult::SingleUnique((x, y), n)) => {
                write!(f, "({}, {}) must be {}, as other solutions would be ambiguous", x + 1, y + 1, n,)
            }
            UniqueRequirement(UrResult::IntraCompartmentUnique((x, y), n)) => {
                write!(f, "({}, {}) cannot be {}, as it would cause ambiguous solutions", x + 1, y + 1, n,)
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
                write!(f, "({}, {}) cannot be {}, as it would cause ambiguous solutions", x + 1, y + 1, n,)
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
                    x + 1,
                    y + 1,
                    n,
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
