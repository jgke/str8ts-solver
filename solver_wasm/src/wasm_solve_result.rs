use crate::wasm_grid::WasmGrid;
use crate::wasm_validation_result::WasmValidationResult;
use serde::{Deserialize, Serialize};
use solver::grid::Point;
use solver::solver::{SolveMetadata, SolveResults, SolveType};
use solver::strats::UrResult;
use std::collections::HashSet;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct WasmSolveMetadata {
    pub colors: Vec<Vec<(Point, u8)>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum WasmUrResult {
    SingleUnique(Point, u8),
    IntraCompartmentUnique(Point, u8),
    ClosedSetCompartment(Vec<Point>, u8),
    SingleCellWouldBecomeFree(Point, u8),
    UrSetti(Vec<Point>, bool, u8),
    SolutionCausesClosedSets(Point, u8),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum WasmSolveType {
    UpdateImpossibles,
    Singles,
    Stranded,
    DefiniteMinMax,
    RequiredRange,
    Sets(usize),
    RequiredAndForbidden,
    RowColBrute,
    Setti(HashSet<u8>),
    YWing(Point, u8),
    Fish(usize),
    Medusa,
    UniqueRequirement(WasmUrResult),
    StartGuess(Point, u8),
    GuessStep(
        Point,
        u8,
        Vec<(WasmGrid, WasmSolveResult, String)>,
        WasmGrid,
    ),
    EndGuess(WasmValidationResult),
    PuzzleSolved,
    EnumerateSolutions,
    OutOfBasicStrats,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WasmSolveResult {
    pub ty: WasmSolveType,
    pub meta: WasmSolveMetadata,
}

impl From<SolveMetadata> for WasmSolveMetadata {
    fn from(value: SolveMetadata) -> Self {
        WasmSolveMetadata {
            colors: value.colors,
        }
    }
}

impl From<WasmSolveMetadata> for SolveMetadata {
    fn from(value: WasmSolveMetadata) -> Self {
        SolveMetadata {
            colors: value.colors,
        }
    }
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

impl From<SolveType> for WasmSolveType {
    fn from(value: SolveType) -> Self {
        match value {
            SolveType::UpdateImpossibles => WasmSolveType::UpdateImpossibles,
            SolveType::Singles => WasmSolveType::Singles,
            SolveType::Stranded => WasmSolveType::Stranded,
            SolveType::DefiniteMinMax => WasmSolveType::DefiniteMinMax,
            SolveType::RequiredRange => WasmSolveType::RequiredRange,
            SolveType::Sets(n) => WasmSolveType::Sets(n),
            SolveType::RequiredAndForbidden => WasmSolveType::RequiredAndForbidden,
            SolveType::RowColBrute => WasmSolveType::RowColBrute,
            SolveType::Setti(set) => WasmSolveType::Setti(set.into()),
            SolveType::YWing(pos, n) => WasmSolveType::YWing(pos, n),
            SolveType::Fish(n) => WasmSolveType::Fish(n),
            SolveType::Medusa => WasmSolveType::Medusa,
            SolveType::UniqueRequirement(res) => WasmSolveType::UniqueRequirement(res.into()),
            SolveType::StartGuess((x, y), n) => WasmSolveType::StartGuess((x, y), n),
            SolveType::GuessStep((x, y), n, steps, grid) => WasmSolveType::GuessStep(
                (x, y),
                n,
                steps
                    .iter()
                    .cloned()
                    .map(|(l, r)| (l.into(), r.clone().into(), r.to_string()))
                    .collect(),
                grid.into(),
            ),
            SolveType::EndGuess(end) => WasmSolveType::EndGuess(end.into()),
            SolveType::PuzzleSolved => WasmSolveType::PuzzleSolved,
            SolveType::EnumerateSolutions => WasmSolveType::EnumerateSolutions,
            SolveType::OutOfBasicStrats => WasmSolveType::OutOfBasicStrats,
        }
    }
}

impl From<WasmSolveType> for SolveType {
    fn from(value: WasmSolveType) -> Self {
        match value {
            WasmSolveType::UpdateImpossibles => SolveType::UpdateImpossibles,
            WasmSolveType::Singles => SolveType::Singles,
            WasmSolveType::Stranded => SolveType::Stranded,
            WasmSolveType::DefiniteMinMax => SolveType::DefiniteMinMax,
            WasmSolveType::RequiredRange => SolveType::RequiredRange,
            WasmSolveType::Sets(n) => SolveType::Sets(n),
            WasmSolveType::RequiredAndForbidden => SolveType::RequiredAndForbidden,
            WasmSolveType::RowColBrute => SolveType::RowColBrute,
            WasmSolveType::Setti(set) => SolveType::Setti(set.into()),
            WasmSolveType::YWing(pos, n) => SolveType::YWing(pos, n),
            WasmSolveType::Fish(n) => SolveType::Fish(n),
            WasmSolveType::Medusa => SolveType::Medusa,
            WasmSolveType::UniqueRequirement(res) => SolveType::UniqueRequirement(res.into()),
            WasmSolveType::StartGuess((x, y), n) => SolveType::StartGuess((x, y), n),
            WasmSolveType::GuessStep((x, y), n, steps, grid) => SolveType::GuessStep(
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
            WasmSolveType::EndGuess(res) => SolveType::EndGuess(res.into()),
            WasmSolveType::PuzzleSolved => SolveType::PuzzleSolved,
            WasmSolveType::EnumerateSolutions => SolveType::EnumerateSolutions,
            WasmSolveType::OutOfBasicStrats => SolveType::OutOfBasicStrats,
        }
    }
}

impl From<SolveResults> for WasmSolveResult {
    fn from(value: SolveResults) -> Self {
        WasmSolveResult {
            ty: value.ty.into(),
            meta: value.meta.into(),
        }
    }
}

impl From<WasmSolveResult> for SolveResults {
    fn from(value: WasmSolveResult) -> Self {
        SolveResults {
            ty: value.ty.into(),
            meta: value.meta.into(),
        }
    }
}
