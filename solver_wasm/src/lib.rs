mod wasm_cell;
mod wasm_difficulty;
mod wasm_grid;
mod wasm_solve_result;
mod wasm_validation_result;

use crate::wasm_difficulty::WasmDifficulty;
use crate::wasm_grid::WasmGrid;
use crate::wasm_solve_result::WasmSolveResult;
use serde::{Deserialize, Serialize};
use solver::solver::{SolveResults, SolveType, ValidationError, ValidationResult, solve_round};
use solver::{generator, grid};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn parse(puzzle: Vec<String>) -> Result<JsValue, JsValue> {
    let res: Result<WasmGrid, String> = grid::Grid::parse(puzzle).map(Into::into);

    Ok(serde_wasm_bindgen::to_value(&res)?)
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct GenerateInput {
    pub size: u8,
    pub blocker_count: u8,
    pub blocker_num_count: u8,
    pub target_difficulty: u8,
    pub symmetric: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct GenerateOutput {
    pub grid: WasmGrid,
    pub grid_str: String,
}

#[wasm_bindgen]
pub fn generate(input: JsValue) -> Result<JsValue, JsValue> {
    let input: GenerateInput = serde_wasm_bindgen::from_value(input)?;
    let grid = generator::generator(
        input.size.into(),
        (input.blocker_count + input.blocker_num_count).into(),
        input.blocker_num_count.into(),
        input.target_difficulty.into(),
        input.symmetric,
    );

    let grid_str = grid.to_string();
    let out = GenerateOutput {
        grid: grid.into(),
        grid_str,
    };

    Ok(serde_wasm_bindgen::to_value(&out)?)
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct SolveOneReturn {
    grid: WasmGrid,
    res_display: Result<String, String>,
    res: Result<WasmSolveResult, String>,
    difficulty: usize,
}

#[wasm_bindgen]
pub fn solve_one(input: JsValue) -> Result<JsValue, JsValue> {
    let grid: WasmGrid = serde_wasm_bindgen::from_value(input)?;
    let mut grid: grid::Grid = grid.into();
    let res = solve_round(&mut grid, true);
    let difficulty = res.as_ref().map(|res| res.ty.difficulty()).unwrap_or(0);
    Ok(serde_wasm_bindgen::to_value(&SolveOneReturn {
        grid: grid.into(),
        res_display: res
            .as_ref()
            .map(|ok| ok.to_string())
            .map_err(|err| err.to_string()),
        res: res.map(|ok| ok.into()).map_err(|err| err.to_string()),
        difficulty,
    })?)
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct SolveReturn {
    res: Vec<SolveOneReturn>,
}

#[wasm_bindgen]
pub fn solve(input: JsValue, use_guesses: bool) -> Result<JsValue, JsValue> {
    let grid: WasmGrid = serde_wasm_bindgen::from_value(input)?;
    let mut grid: grid::Grid = grid.into();

    let mut res = Vec::new();

    loop {
        match solve_round(&mut grid, use_guesses) {
            Ok(strat) => {
                let difficulty = strat.ty.difficulty();
                let was_solved = strat.ty == SolveType::PuzzleSolved;
                res.push(SolveOneReturn {
                    grid: grid.clone().into(),
                    res_display: Ok(strat.to_string()),
                    res: Ok(strat.into()),
                    difficulty,
                });
                if was_solved {
                    break;
                }
            }
            Err(ValidationResult {
                ty: ValidationError::OutOfStrats,
                meta: _,
            }) => {
                break;
            }
            Err(e) => {
                res.push(SolveOneReturn {
                    grid: grid.clone().into(),
                    res: Err(e.to_string()),
                    res_display: Err(e.to_string()),
                    difficulty: 0,
                });
                break;
            }
        }
    }

    Ok(serde_wasm_bindgen::to_value(&SolveReturn { res })?)
}

#[wasm_bindgen]
pub fn puzzle_difficulty(input: JsValue) -> Result<JsValue, JsValue> {
    let history: Vec<WasmSolveResult> = serde_wasm_bindgen::from_value(input)?;
    let history: Vec<SolveResults> = history.into_iter().map(|item| item.into()).collect();
    let difficulty: WasmDifficulty = solver::difficulty::puzzle_difficulty(
        &history.iter().map(|res| &res.ty).collect::<Vec<_>>(),
    )
    .into();

    Ok(serde_wasm_bindgen::to_value(&difficulty)?)
}
