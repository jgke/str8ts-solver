use crate::wasm_cell::WasmCell;
use serde::{Deserialize, Serialize};
use solver::grid;
use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WasmGrid {
    pub cells: Vec<Vec<WasmCell>>,
    pub x: usize,
    pub y: usize,
    pub row_requirements: Vec<HashSet<u8>>,
    pub col_requirements: Vec<HashSet<u8>>,
    pub row_forbidden: Vec<HashSet<u8>>,
    pub col_forbidden: Vec<HashSet<u8>>,
}

impl From<grid::Grid> for WasmGrid {
    fn from(value: grid::Grid) -> Self {
        WasmGrid {
            cells: value
                .cells
                .into_iter()
                .map(|row| row.into_iter().map(Into::into).collect())
                .collect(),
            x: value.x,
            y: value.y,
            row_requirements: value.row_requirements.into_iter().map(Into::into).collect(),
            col_requirements: value.col_requirements.into_iter().map(Into::into).collect(),
            row_forbidden: value.row_forbidden.into_iter().map(Into::into).collect(),
            col_forbidden: value.col_forbidden.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<WasmGrid> for grid::Grid {
    fn from(value: WasmGrid) -> Self {
        grid::Grid {
            cells: value
                .cells
                .into_iter()
                .map(|row| row.into_iter().map(Into::into).collect())
                .collect(),
            x: value.x,
            y: value.y,
            row_requirements: value.row_requirements.into_iter().map(Into::into).collect(),
            col_requirements: value.col_requirements.into_iter().map(Into::into).collect(),
            row_forbidden: value.row_forbidden.into_iter().map(Into::into).collect(),
            col_forbidden: value.col_forbidden.into_iter().map(Into::into).collect(),
        }
    }
}
