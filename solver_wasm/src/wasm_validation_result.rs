use crate::wasm_solve_result::WasmSolveMetadata;
use serde::{Deserialize, Serialize};
use solver::grid::Point;
use solver::solver::{ValidationError, ValidationResult};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum WasmValidationError {
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
    OutOfStrats,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WasmValidationResult {
    pub ty: WasmValidationError,
    pub meta: WasmSolveMetadata,
}

impl From<ValidationError> for WasmValidationError {
    #[rustfmt::skip]
    fn from(value: ValidationError) -> Self {
        match value {
            ValidationError::EmptyCell { pos } => WasmValidationError::EmptyCell { pos },
            ValidationError::Conflict { pos1, pos2, val } => WasmValidationError::Conflict { pos1, pos2, val },
            ValidationError::Sequence { vertical, top_left, range, missing, } => WasmValidationError::Sequence { vertical, top_left, range, missing, },
            ValidationError::SequenceTooLarge { vertical, top_left, contains, max_ranges, } => WasmValidationError::SequenceTooLarge { vertical, top_left, contains, max_ranges, },
            ValidationError::RequirementBlockerConflict { vertical, index, number, } => WasmValidationError::RequirementBlockerConflict { vertical, index, number, },
            ValidationError::RequiredNumberMissing { vertical, index, number, } => WasmValidationError::RequiredNumberMissing { vertical, index, number, },
            ValidationError::BlockedNumberPresent { vertical, index, number, } => WasmValidationError::BlockedNumberPresent { vertical, index, number, },
            ValidationError::Ambiguous { cells, } => WasmValidationError::Ambiguous { cells, },
            ValidationError::OutOfStrats => WasmValidationError::OutOfStrats,
        }
    }
}

impl From<WasmValidationError> for ValidationError {
    #[rustfmt::skip]
    fn from(value: WasmValidationError) -> Self {
        match value {
            WasmValidationError::EmptyCell { pos } => ValidationError::EmptyCell { pos },
            WasmValidationError::Conflict { pos1, pos2, val } => ValidationError::Conflict { pos1, pos2, val },
            WasmValidationError::Sequence { vertical, top_left, range, missing, } => ValidationError::Sequence { vertical, top_left, range, missing, },
            WasmValidationError::SequenceTooLarge { vertical, top_left, contains, max_ranges, } => ValidationError::SequenceTooLarge { vertical, top_left, contains, max_ranges, },
            WasmValidationError::RequirementBlockerConflict { vertical, index, number, } => ValidationError::RequirementBlockerConflict { vertical, index, number, },
            WasmValidationError::RequiredNumberMissing { vertical, index, number, } => ValidationError::RequiredNumberMissing { vertical, index, number, },
            WasmValidationError::BlockedNumberPresent { vertical, index, number, } => ValidationError::BlockedNumberPresent { vertical, index, number, },
            WasmValidationError::Ambiguous { cells, } => ValidationError::Ambiguous { cells, },
            WasmValidationError::OutOfStrats => ValidationError::OutOfStrats,
        }
    }
}

impl From<ValidationResult> for WasmValidationResult {
    fn from(value: ValidationResult) -> Self {
        WasmValidationResult {
            ty: value.ty.into(),
            meta: value.meta.into(),
        }
    }
}

impl From<WasmValidationResult> for ValidationResult {
    fn from(value: WasmValidationResult) -> Self {
        ValidationResult {
            ty: value.ty.into(),
            meta: value.meta.into(),
        }
    }
}
