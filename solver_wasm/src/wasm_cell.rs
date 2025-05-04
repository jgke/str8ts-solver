use serde::{Deserialize, Serialize};
use solver::grid;
use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum WasmCell {
    Requirement(u8),
    Solution(u8),
    Blocker(u8),
    Indeterminate(HashSet<u8>),
    Black,
}

impl From<grid::Cell> for WasmCell {
    fn from(value: grid::Cell) -> Self {
        match value {
            grid::Cell::Requirement(num) => WasmCell::Requirement(num),
            grid::Cell::Solution(num) => WasmCell::Solution(num),
            grid::Cell::Blocker(num) => WasmCell::Blocker(num),
            grid::Cell::Indeterminate(set) => WasmCell::Indeterminate(set.into()),
            grid::Cell::Black => WasmCell::Black,
        }
    }
}

impl From<WasmCell> for grid::Cell {
    fn from(value: WasmCell) -> Self {
        match value {
            WasmCell::Requirement(num) => grid::Cell::Requirement(num),
            WasmCell::Solution(num) => grid::Cell::Solution(num),
            WasmCell::Blocker(num) => grid::Cell::Blocker(num),
            WasmCell::Indeterminate(set) => grid::Cell::Indeterminate(set.into()),
            WasmCell::Black => grid::Cell::Black,
        }
    }
}
