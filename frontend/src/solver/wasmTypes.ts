import * as mod from "../../../solver_wasm/target/pkg-web";

export type Point = [number, number];
export type WasmCell =
  | { Requirement: number }
  | { Solution: number }
  | { Blocker: number }
  | { Indeterminate: number[] }
  | "Black";

export interface WasmGrid {
  cells: WasmCell[][];
  x: number;
  y: number;
  row_requirements: number[][];
  col_requirements: number[][];
  row_forbidden: number[][];
  col_forbidden: number[][];
}

export type WasmOk<T> = { Ok: T };
export type WasmErr<K> = { Err: K };
export type WasmResult<T, K> = WasmOk<T> | WasmErr<K>;

export function isOk<T, K>(res: WasmResult<T, K>): res is WasmOk<T> {
  return !!(res as WasmOk<T>).Ok;
}

export function isErr<T, K>(res: WasmResult<T, K>): res is WasmErr<K> {
  return !isOk(res);
}

export type WasmUrResult =
  | { SingleUnique: [Point, number] }
  | { IntraCompartmentUnique: [Point, number] }
  | { ClosedSetCompartment: [Point[], number] }
  | { SingleCellWouldBecomeFree: [Point, number] }
  | { UrSetti: [Point[], boolean, number] }
  | { SolutionCausesClosedSets: [Point, number] };

export interface WasmSolveMetadata {
  colors: [Point, number][][];
}

export type WasmSolveType =
  | "UpdateImpossibles"
  | "Singles"
  | "Stranded"
  | "DefiniteMinMax"
  | "RequiredRange"
  | { Sets: number }
  | "RequiredAndForbidden"
  | "RowColBrute"
  | { Setti: number[] }
  | { YWing: [Point, number] }
  | { Fish: number }
  | { Medusa: [[Point, number][], [Point, number][]] }
  | { UniqueRequirement: WasmUrResult }
  | { StartGuess: [Point, number] }
  | { GuessStep: [Point, number, [WasmGrid, WasmSolveResult, string][], WasmGrid] }
  | { EndGuess: WasmValidationResult }
  | "PuzzleSolved"
  | "EnumerateSolutions"
  | "OutOfBasicStrats";

export interface WasmSolveResult {
  ty: WasmSolveType;
  meta: WasmSolveMetadata;
}

export type WasmValidationError =
  | { EmptyCell: { pos: Point } }
  | { Conflict: { pos1: Point; pos2: Point; val: number } }
  | { Sequence: { vertical: boolean; top_left: Point; range: Point; missing: number } }
  | {
      SequenceTooLarge: {
        vertical: boolean;
        top_left: Point;
        contains: Point;
        max_ranges: [Point, Point];
      };
    }
  | { RequirementBlockerConflict: { vertical: boolean; index: number; number: number } }
  | { RequiredNumberMissing: { vertical: boolean; index: number; number: number } }
  | { BlockedNumberPresent: { vertical: boolean; index: number; number: number } }
  | { Ambiguous: { cells: Point[] } }
  | "OutOfStrats";

export interface WasmValidationResult {
  ty: WasmValidationError;
  meta: WasmSolveMetadata;
}

export interface WasmDifficulty {
  star_count: number;
  move_count: number;
  basic_reductions: boolean;
  min_max_reductions: boolean;
  cross_compartment_ranges: boolean;
  maintain_reqs_and_blocks: boolean;
  sets: boolean;
  setti: boolean;
  y_wing: boolean;
  x_wing: boolean;
  swordfish: boolean;
  n_fish: number;
  medusa: boolean;
  unique_requirement: boolean;
  short_guess_count: number;
  long_guess_count: number;
}

export interface WasmSolveOneReturn {
  difficulty: number;
  grid: WasmGrid;
  res: WasmResult<WasmSolveResult, WasmValidationResult>;
  res_display: WasmResult<string, string>;
}

export interface WasmSolveReturn {
  res: WasmSolveOneReturn[];
}

export interface WasmGeneratorInput {
  size: number;
  blocker_count: number;
  blocker_num_count: number;
  symmetric: boolean;
  target_difficulty: number;
}

export function parse(input: string[]): WasmResult<WasmGrid, string> {
  return mod.parse(input);
}

export function solve_one(input: WasmGrid): WasmSolveOneReturn {
  return mod.solve_one(input);
}

export function solve(input: WasmGrid, useGuessing: boolean): WasmSolveReturn {
  return mod.solve(input, useGuessing);
}

export function puzzle_difficulty(history: WasmSolveResult[]): WasmDifficulty {
  return mod.puzzle_difficulty(history);
}

const generatorWorker = new Worker(new URL("generatorWorker.js", import.meta.url));
generatorWorker.onerror = console.warn;

export async function generate(input: WasmGeneratorInput): Promise<{ grid: WasmGrid; grid_str: string }> {
  return new Promise<{ grid: WasmGrid; grid_str: string }>((resolve) => {
    generatorWorker.onmessage = (ev) => {
      resolve(ev.data);
    };
    generatorWorker.onerror = (e) => {
      throw e;
    };
    generatorWorker.postMessage(input);
  });
}
