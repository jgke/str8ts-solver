use serde::{Deserialize, Serialize};
use solver::grid::Point;
use solver::solver::ValidationResult;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum WasmValidationResult {
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

impl From<ValidationResult> for WasmValidationResult {
    #[rustfmt::skip]
    fn from(value: ValidationResult) -> Self {
        match value {
            ValidationResult::EmptyCell { pos } => WasmValidationResult::EmptyCell { pos },
            ValidationResult::Conflict { pos1, pos2, val } => WasmValidationResult::Conflict { pos1, pos2, val },
            ValidationResult::Sequence { vertical, top_left, range, missing, } => WasmValidationResult::Sequence { vertical, top_left, range, missing, },
            ValidationResult::SequenceTooLarge { vertical, top_left, contains, max_ranges, } => WasmValidationResult::SequenceTooLarge { vertical, top_left, contains, max_ranges, },
            ValidationResult::RequirementBlockerConflict { vertical, index, number, } => WasmValidationResult::RequirementBlockerConflict { vertical, index, number, },
            ValidationResult::RequiredNumberMissing { vertical, index, number, } => WasmValidationResult::RequiredNumberMissing { vertical, index, number, },
            ValidationResult::BlockedNumberPresent { vertical, index, number, } => WasmValidationResult::BlockedNumberPresent { vertical, index, number, },
            ValidationResult::Ambiguous { cells, } => WasmValidationResult::Ambiguous { cells, },
            ValidationResult::OutOfStrats => WasmValidationResult::OutOfStrats,
        }
    }
}

impl From<WasmValidationResult> for ValidationResult {
    #[rustfmt::skip]
    fn from(value: WasmValidationResult) -> Self {
        match value {
            WasmValidationResult::EmptyCell { pos } => ValidationResult::EmptyCell { pos },
            WasmValidationResult::Conflict { pos1, pos2, val } => ValidationResult::Conflict { pos1, pos2, val },
            WasmValidationResult::Sequence { vertical, top_left, range, missing, } => ValidationResult::Sequence { vertical, top_left, range, missing, },
            WasmValidationResult::SequenceTooLarge { vertical, top_left, contains, max_ranges, } => ValidationResult::SequenceTooLarge { vertical, top_left, contains, max_ranges, },
            WasmValidationResult::RequirementBlockerConflict { vertical, index, number, } => ValidationResult::RequirementBlockerConflict { vertical, index, number, },
            WasmValidationResult::RequiredNumberMissing { vertical, index, number, } => ValidationResult::RequiredNumberMissing { vertical, index, number, },
            WasmValidationResult::BlockedNumberPresent { vertical, index, number, } => ValidationResult::BlockedNumberPresent { vertical, index, number, },
            WasmValidationResult::Ambiguous { cells, } => ValidationResult::Ambiguous { cells, },
            WasmValidationResult::OutOfStrats => ValidationResult::OutOfStrats,
        }
    }
}
