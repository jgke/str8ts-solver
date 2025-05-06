import {
  isErr,
  WasmCell,
  WasmGrid,
  WasmResult,
  WasmSolveOneReturn,
  WasmSolveReturn,
  parse as wasmParse,
  solve_one as wasmSolveOne,
  solve as wasmSolve,
  puzzle_difficulty as wasmPuzzleDifficulty,
  WasmSolveResult,
  WasmDifficulty,
} from "./wasmTypes.ts";

export type Cell =
  | { ty: "Requirement"; num: number }
  | { ty: "Solution"; num: number }
  | { ty: "Blocker"; num: number }
  | { ty: "Indeterminate"; set: Set<number> }
  | { ty: "Black" };

export interface Grid {
  cells: Cell[][];
  x: number;
  y: number;
  row_requirements: Set<number>[];
  col_requirements: Set<number>[];
  row_forbidden: Set<number>[];
  col_forbidden: Set<number>[];
}

function cell_from_wasm(input: WasmCell): Cell {
  if (input === "Black") return { ty: "Black" };
  if ("Requirement" in input) return { ty: "Requirement", num: input.Requirement };
  if ("Solution" in input) return { ty: "Solution", num: input.Solution };
  if ("Blocker" in input) return { ty: "Blocker", num: input.Blocker };
  if ("Indeterminate" in input) return { ty: "Indeterminate", set: new Set(input.Indeterminate) };
  throw input;
}

function cell_to_wasm(input: Cell): WasmCell {
  if (input.ty === "Black") return "Black";
  if (input.ty === "Requirement") return { Requirement: input.num };
  if (input.ty === "Solution") return { Solution: input.num };
  if (input.ty === "Blocker") return { Blocker: input.num };
  if (input.ty === "Indeterminate") return { Indeterminate: [...input.set] };
  throw input;
}

export function gridFromWasm(grid: WasmGrid): Grid {
  const { cells, col_forbidden, col_requirements, row_forbidden, row_requirements, x, y } = grid;
  return {
    cells: cells.map((row) => row.map(cell_from_wasm)),
    x,
    y,
    row_requirements: row_requirements.map((row) => new Set(row)),
    col_requirements: col_requirements.map((row) => new Set(row)),
    row_forbidden: row_forbidden.map((row) => new Set(row)),
    col_forbidden: col_forbidden.map((row) => new Set(row)),
  };
}

function gridToWasm(grid: Grid): WasmGrid {
  const { cells, col_forbidden, col_requirements, row_forbidden, row_requirements, x, y } = grid;
  return {
    cells: cells.map((row) => row.map(cell_to_wasm)),
    x,
    y,
    row_requirements: row_requirements.map((row) => [...row]),
    col_requirements: col_requirements.map((row) => [...row]),
    row_forbidden: row_forbidden.map((row) => [...row]),
    col_forbidden: col_forbidden.map((row) => [...row]),
  };
}

export function parse(input: string[]): WasmResult<Grid, string> {
  const out: WasmResult<WasmGrid, string> = wasmParse(input);
  if (isErr(out)) {
    return { Err: out.Err };
  }
  return { Ok: gridFromWasm(out.Ok) };
}

interface SolveOneResult {
  difficulty: number;
  grid: Grid;
  res: WasmResult<WasmSolveResult, string>;
  resDisplay: WasmResult<string, string>;
}

function solveOneResultFromWasm(out: WasmSolveOneReturn): SolveOneResult {
  return { grid: gridFromWasm(out.grid), difficulty: out.difficulty, res: out.res, resDisplay: out.res_display };
}

export function solve_one(input: Grid): SolveOneResult {
  const grid = gridToWasm(input);
  const out: WasmSolveOneReturn = wasmSolveOne(grid);
  return solveOneResultFromWasm(out);
}

interface SolveResult {
  res: SolveOneResult[];
}

export function solve(input: Grid, useGuessing: boolean): SolveResult {
  const grid = gridToWasm(input);
  const out: WasmSolveReturn = wasmSolve(grid, useGuessing);
  return { res: out.res.map(solveOneResultFromWasm) };
}

export function puzzleDifficulty(history: WasmSolveResult[]): WasmDifficulty {
  return wasmPuzzleDifficulty(history);
}

export function getColors(row: WasmResult<WasmSolveResult, string>): number[][][] | null {
  if ("Err" in row) return null;
  if (typeof row.Ok === "object" && "Medusa" in row.Ok) {
    const [left, right] = row.Ok.Medusa;
    const grid = [...Array(9)].map(() => [...Array(9)].map(() => [...Array(10)].map(() => 0)));

    left.forEach(([[x, y], n]) => {
      grid[y][x][n] = 1;
    });
    right.forEach(([[x, y], n]) => {
      grid[y][x][n] = 2;
    });
    return grid;
  }
  return null;
}
