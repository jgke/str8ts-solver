use serde::{Deserialize, Serialize};
use solver::difficulty::Difficulty;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasmDifficulty {
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
    pub unique_requirement: bool,
    pub short_guess_count: usize,
    pub long_guess_count: usize,
}

impl From<Difficulty> for WasmDifficulty {
    fn from(value: Difficulty) -> Self {
        WasmDifficulty {
            star_count: value.star_count,
            move_count: value.move_count,
            basic_reductions: value.basic_reductions,
            min_max_reductions: value.min_max_reductions,
            cross_compartment_ranges: value.cross_compartment_ranges,
            maintain_reqs_and_blocks: value.maintain_reqs_and_blocks,
            sets: value.sets,
            setti: value.setti,
            y_wing: value.y_wing,
            x_wing: value.x_wing,
            swordfish: value.swordfish,
            n_fish: value.n_fish,
            unique_requirement: value.unique_requirement,
            short_guess_count: value.short_guess_count,
            long_guess_count: value.long_guess_count,
        }
    }
}
