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
    pub x_wing: bool,
    pub swordfish: bool,
    pub medusa: bool,
    pub n_fish: usize,
    pub unique_requirement_single: bool,
    pub unique_requirement_count: usize,
    pub short_chain_count: usize,
    pub long_chain_count: usize,
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
            x_wing: value.x_wing,
            swordfish: value.swordfish,
            medusa: value.medusa,
            n_fish: value.n_fish,
            unique_requirement_single: value.unique_requirement_single,
            unique_requirement_count: value.unique_requirement_count,
            short_chain_count: value.short_chain_count,
            long_chain_count: value.long_chain_count,
        }
    }
}
