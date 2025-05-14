use crate::bitset::BitSet;
use crate::grid::{Grid, Point};
use crate::solver::SolveType::*;
use crate::solver::ValidationError::*;
use crate::strats;
use crate::strats::UrResult;
use crate::validator::validate;
use itertools::intersperse;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Strategy {
    /* todo: a lot of these have dependencies between them... */
    UpdateImpossibles,
    Singles,
    Stranded,
    DefiniteMinMax,
    RequiredRange,
    Sets,
    RequiredAndForbidden,
    RowColBrute,
    Setti,
    YWing,
    Fish,
    Medusa,
    UniqueRequirement,
    UniqueRequirementGuess,
    Guess,
    EnumerateSolutions,
}

impl Strategy {
    pub fn difficulty(&self) -> usize {
        match self {
            Strategy::UpdateImpossibles => 1,
            Strategy::Stranded => 1,
            Strategy::DefiniteMinMax => 2,
            Strategy::Singles => 2,
            Strategy::RequiredRange => 4,
            Strategy::Sets => 4,
            Strategy::RequiredAndForbidden => 5,
            Strategy::RowColBrute => 5,
            Strategy::Setti => 5,
            Strategy::YWing => 5,
            Strategy::Fish => 5,
            Strategy::Medusa => 6,
            Strategy::UniqueRequirement => 6,
            Strategy::UniqueRequirementGuess => 7,
            Strategy::Guess => 7,
            Strategy::EnumerateSolutions => 7,
        }
    }
}

const ALL_STRATEGIES: [Strategy; 16] = [
    Strategy::UpdateImpossibles,
    Strategy::Singles,
    Strategy::Stranded,
    Strategy::DefiniteMinMax,
    Strategy::RequiredRange,
    Strategy::Sets,
    Strategy::RequiredAndForbidden,
    Strategy::RowColBrute,
    Strategy::Setti,
    Strategy::YWing,
    Strategy::Fish,
    Strategy::Medusa,
    Strategy::UniqueRequirement,
    Strategy::UniqueRequirementGuess,
    Strategy::Guess,
    Strategy::EnumerateSolutions,
];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StrategyList {
    strats: HashMap<Strategy, bool>,
}

impl StrategyList {
    pub fn new(strats: &[Strategy]) -> StrategyList {
        StrategyList {
            strats: strats.iter().map(|&strat| (strat, true)).collect(),
        }
    }

    pub fn all() -> StrategyList {
        StrategyList::new(&ALL_STRATEGIES)
    }

    pub fn for_difficulty(difficulty: usize) -> StrategyList {
        StrategyList::new(
            &ALL_STRATEGIES
                .iter()
                .copied()
                .filter(|strat| strat.difficulty() <= difficulty)
                .collect::<Vec<_>>(),
        )
    }

    pub fn basic() -> StrategyList {
        StrategyList::new(&[
            Strategy::UpdateImpossibles,
            Strategy::Singles,
            Strategy::Stranded,
            Strategy::DefiniteMinMax,
            Strategy::RequiredRange,
            Strategy::Sets,
        ])
    }

    pub fn fast() -> StrategyList {
        StrategyList::basic().except(&[Strategy::Sets])
    }

    pub fn no_guesses() -> StrategyList {
        StrategyList::all().except(&[
            Strategy::UniqueRequirementGuess,
            Strategy::Guess,
            Strategy::EnumerateSolutions,
        ])
    }

    pub fn except(&self, without_strats: &[Strategy]) -> StrategyList {
        let mut strats = self.strats.clone();
        for &strat in without_strats {
            strats.insert(strat, false);
        }
        StrategyList { strats }
    }

    pub fn has(&self, strat: Strategy) -> bool {
        *self.strats.get(&strat).unwrap_or(&false)
    }
}

pub type StrategyReturn = Result<Option<SolveResults>, ValidationResult>;

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
        run_strat!(
            strats,
            Strategy::UpdateImpossibles,
            strats::update_impossibles(grid)
        );
        run_strat!(strats, Strategy::Singles, strats::singles(grid));
        run_strat!(strats, Strategy::Stranded, strats::stranded(grid));
        run_strat!(
            strats,
            Strategy::DefiniteMinMax,
            strats::definite_min_max(grid)
        );
        run_strat!(
            strats,
            Strategy::RequiredRange,
            strats::required_range(grid)
        );

        /* advanced strats */
        run_strat!(
            strats,
            Strategy::RequiredAndForbidden,
            strats::update_required_and_forbidden(grid)
        );
        run_strat!(strats, Strategy::Setti, strats::setti(grid));
        run_strat!(strats, Strategy::RowColBrute, strats::row_col_brute(grid));
        run_strat!(strats, Strategy::YWing, strats::y_wing(grid));
        run_strat!(strats, Strategy::Fish, strats::fish(grid));

        run_strat!(strats, Strategy::Medusa, strats::medusa(grid));

        run_strat!(
            strats,
            Strategy::UniqueRequirement,
            strats::unique_requirement(grid)
        );
        run_strat!(
            strats,
            Strategy::UniqueRequirementGuess,
            strats::unique_requirement_guess(grid)
        );

        run_strat!(strats, Strategy::Guess, strats::guess(grid));

        run_strat!(
            strats,
            Strategy::EnumerateSolutions,
            strats::enumerate_solutions(grid)
        );

        Err(OutOfStrats.into())
    })();
    strats::trivial(grid);
    validate(grid)?;
    res
}

pub fn run_fast_basic(grid: &mut Grid) -> Result<SolveType, ValidationResult> {
    run_strat(grid, &StrategyList::fast()).map(|t| t.ty)
}

pub fn solve_round(
    grid: &mut Grid,
    enable_guesses: bool,
) -> Result<SolveResults, ValidationResult> {
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
