use solver::solver::SolveResults;

#[derive(Debug, Clone)]
pub struct Difficulty {
    pub star_count: usize,
    pub move_count: usize,
    pub basic_reductions: bool,
    pub min_max_reductions: bool,
    pub cross_compartment_ranges: bool,
    pub sets: bool,
    pub setti: bool,
    pub x_wing: bool,
    pub swordfish: bool,
    pub medusa: bool,
    pub n_fish: usize,
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
        setti: history.iter().any(|e| matches!(e, SolveResults::Setti)),
        x_wing: history.iter().any(|e| matches!(e, SolveResults::Fish(2))),
        swordfish: history.iter().any(|e| matches!(e, SolveResults::Fish(3))),
        medusa: history.iter().any(|e| matches!(e, SolveResults::Fish(4))),
        n_fish: history
            .iter()
            .map(|e| if let SolveResults::Fish(n) = e { *n } else { 0 })
            .max()
            .unwrap_or(0),
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
