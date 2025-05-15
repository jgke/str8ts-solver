use crate::solve_result::{SolveResults, ValidationResult};
use std::collections::HashMap;

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
