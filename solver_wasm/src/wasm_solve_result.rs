use crate::wasm_grid::WasmGrid;
use crate::wasm_validation_result::WasmValidationResult;
use serde::{Deserialize, Serialize};
use solver::solver::SolveResults;
use solver::strats::UrResult;
use std::collections::HashSet;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum WasmUrResult {
    SingleUnique((usize, usize), u8),
    IntraCompartmentUnique((usize, usize), u8),
    ClosedSetCompartment(Vec<(usize, usize)>, u8),
    SingleCellWouldBecomeFree((usize, usize), u8),
    UrSetti(Vec<(usize, usize)>, bool, u8),
    SolutionCausesClosedSets((usize, usize), u8),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum WasmSolveResult {
    UpdateImpossibles,
    Singles,
    Stranded,
    DefiniteMinMax,
    RequiredRange,
    Sets(usize),
    RequiredAndForbidden,
    RowColBrute,
    Setti(HashSet<u8>),
    Fish(usize),
    SimpleUniqueRequirement(WasmUrResult),
    UniqueRequirement(
        (usize, usize),
        u8,
        Vec<(WasmGrid, WasmSolveResult, String)>,
        WasmGrid,
    ),
    StartChain((usize, usize), u8),
    Chain(
        (usize, usize),
        u8,
        Vec<(WasmGrid, WasmSolveResult, String)>,
        WasmGrid,
    ),
    EndChain(WasmValidationResult),
    PuzzleSolved,
    OutOfBasicStrats,
}

impl From<UrResult> for WasmUrResult {
    fn from(value: UrResult) -> Self {
        match value {
            UrResult::SingleUnique((x, y), n) => WasmUrResult::SingleUnique((x, y), n),
            UrResult::IntraCompartmentUnique((x, y), n) => {
                WasmUrResult::IntraCompartmentUnique((x, y), n)
            }
            UrResult::ClosedSetCompartment(set, n) => WasmUrResult::ClosedSetCompartment(set, n),
            UrResult::SingleCellWouldBecomeFree((x, y), n) => {
                WasmUrResult::SingleCellWouldBecomeFree((x, y), n)
            }
            UrResult::UrSetti(set, b, n) => WasmUrResult::UrSetti(set, b, n),
            UrResult::SolutionCausesClosedSets((x, y), n) => {
                WasmUrResult::SolutionCausesClosedSets((x, y), n)
            }
        }
    }
}

impl From<WasmUrResult> for UrResult {
    fn from(value: WasmUrResult) -> Self {
        match value {
            WasmUrResult::SingleUnique((x, y), n) => UrResult::SingleUnique((x, y), n),
            WasmUrResult::IntraCompartmentUnique((x, y), n) => {
                UrResult::IntraCompartmentUnique((x, y), n)
            }
            WasmUrResult::ClosedSetCompartment(set, n) => UrResult::ClosedSetCompartment(set, n),
            WasmUrResult::SingleCellWouldBecomeFree((x, y), n) => {
                UrResult::SingleCellWouldBecomeFree((x, y), n)
            }
            WasmUrResult::UrSetti(set, b, n) => UrResult::UrSetti(set, b, n),
            WasmUrResult::SolutionCausesClosedSets((x, y), n) => {
                UrResult::SolutionCausesClosedSets((x, y), n)
            }
        }
    }
}

impl From<SolveResults> for WasmSolveResult {
    fn from(value: SolveResults) -> Self {
        match value {
            SolveResults::UpdateImpossibles => WasmSolveResult::UpdateImpossibles,
            SolveResults::Singles => WasmSolveResult::Singles,
            SolveResults::Stranded => WasmSolveResult::Stranded,
            SolveResults::DefiniteMinMax => WasmSolveResult::DefiniteMinMax,
            SolveResults::RequiredRange => WasmSolveResult::RequiredRange,
            SolveResults::Sets(n) => WasmSolveResult::Sets(n),
            SolveResults::RequiredAndForbidden => WasmSolveResult::RequiredAndForbidden,
            SolveResults::RowColBrute => WasmSolveResult::RowColBrute,
            SolveResults::Setti(set) => WasmSolveResult::Setti(set.into()),
            SolveResults::Fish(n) => WasmSolveResult::Fish(n),
            SolveResults::SimpleUniqueRequirement(res) => {
                WasmSolveResult::SimpleUniqueRequirement(res.into())
            }
            SolveResults::UniqueRequirement((x, y), n, steps, grid) => {
                WasmSolveResult::UniqueRequirement(
                    (x, y),
                    n,
                    steps
                        .iter()
                        .cloned()
                        .map(|(l, r)| (l.into(), r.clone().into(), r.to_string()))
                        .collect(),
                    grid.into(),
                )
            }
            SolveResults::StartChain((x, y), n) => WasmSolveResult::StartChain((x, y), n),
            SolveResults::Chain((x, y), n, steps, grid) => WasmSolveResult::Chain(
                (x, y),
                n,
                steps
                    .iter()
                    .cloned()
                    .map(|(l, r)| (l.into(), r.clone().into(), r.to_string()))
                    .collect(),
                grid.into(),
            ),
            SolveResults::EndChain(end) => WasmSolveResult::EndChain(end.into()),
            SolveResults::PuzzleSolved => WasmSolveResult::PuzzleSolved,
            SolveResults::OutOfBasicStrats => WasmSolveResult::OutOfBasicStrats,
        }
    }
}

impl From<WasmSolveResult> for SolveResults {
    fn from(value: WasmSolveResult) -> Self {
        match value {
            WasmSolveResult::UpdateImpossibles => SolveResults::UpdateImpossibles,
            WasmSolveResult::Singles => SolveResults::Singles,
            WasmSolveResult::Stranded => SolveResults::Stranded,
            WasmSolveResult::DefiniteMinMax => SolveResults::DefiniteMinMax,
            WasmSolveResult::RequiredRange => SolveResults::RequiredRange,
            WasmSolveResult::Sets(n) => SolveResults::Sets(n),
            WasmSolveResult::RequiredAndForbidden => SolveResults::RequiredAndForbidden,
            WasmSolveResult::RowColBrute => SolveResults::RowColBrute,
            WasmSolveResult::Setti(set) => SolveResults::Setti(set.into()),
            WasmSolveResult::Fish(n) => SolveResults::Fish(n),
            WasmSolveResult::SimpleUniqueRequirement(res) => {
                SolveResults::SimpleUniqueRequirement(res.into())
            }
            WasmSolveResult::UniqueRequirement((x, y), n, steps, grid) => {
                SolveResults::UniqueRequirement(
                    (x, y),
                    n,
                    Rc::new(
                        steps
                            .iter()
                            .cloned()
                            .map(|(l, r, _)| (l.into(), r.into()))
                            .collect(),
                    ),
                    grid.into(),
                )
            }
            WasmSolveResult::StartChain((x, y), n) => SolveResults::StartChain((x, y), n),
            WasmSolveResult::Chain((x, y), n, steps, grid) => SolveResults::Chain(
                (x, y),
                n,
                Rc::new(
                    steps
                        .iter()
                        .cloned()
                        .map(|(l, r, _)| (l.into(), r.into()))
                        .collect(),
                ),
                grid.into(),
            ),
            WasmSolveResult::EndChain(res) => SolveResults::EndChain(res.into()),
            WasmSolveResult::PuzzleSolved => SolveResults::PuzzleSolved,
            WasmSolveResult::OutOfBasicStrats => SolveResults::OutOfBasicStrats,
        }
    }
}
