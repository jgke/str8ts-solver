import * as mod from "../../../solver_wasm/target/pkg-web";

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
  | { SingleUnique: [[number, number], number] }
  | { IntraCompartmentUnique: [[number, number], number] }
  | { ClosedSetCompartment: [[number, number][], number] }
  | { SingleCellWouldBecomeFree: [[number, number], number] }
  | { UrSetti: [[number, number][], boolean, number] }
  | { SolutionCausesClosedSets: [[number, number], number] };

export type WasmSolveResult =
  | "UpdateImpossibles"
  | "Singles"
  | "Stranded"
  | "DefiniteMinMax"
  | "RequiredRange"
  | { Sets: number }
  | "RequiredAndForbidden"
  | "RowColBrute"
  | { Setti: number[] }
  | { Fish: number }
  | { SimpleUniqueRequirement: WasmUrResult }
  | { UniqueRequirement: [[number, number], number, [WasmGrid, WasmSolveResult, string][], WasmGrid] }
  | { StartChain: [[number, number], number] }
  | { Chain: [[number, number], number, [WasmGrid, WasmSolveResult, string][], WasmGrid] }
  | { EndChain: WasmValidationResult }
  | "PuzzleSolved"
  | "OutOfBasicStrats";

export type WasmValidationResult =
  | { EmptyCell: { pos: [number, number] } }
  | { Conflict: { pos1: [number, number]; pos2: [number, number]; val: number } }
  | { Sequence: { vertical: boolean; top_left: [number, number]; range: [number, number]; missing: number } }
  | {
      SequenceTooLarge: {
        vertical: boolean;
        top_left: [number, number];
        contains: [number, number];
        max_ranges: [[number, number], [number, number]];
      };
    }
  | { RequirementBlockerConflict: { vertical: boolean; index: number; number: number } }
  | { RequiredNumberMissing: { vertical: boolean; index: number; number: number } }
  | { BlockedNumberPresent: { vertical: boolean; index: number; number: number } }
  | { Ambiguous: { cells: [number, number][] } }
  | "OutOfStrats";

export interface WasmDifficulty {
  star_count: number;
  move_count: number;
  basic_reductions: boolean;
  min_max_reductions: boolean;
  cross_compartment_ranges: boolean;
  maintain_reqs_and_blocks: boolean;
  sets: boolean;
  setti: boolean;
  x_wing: boolean;
  swordfish: boolean;
  medusa: boolean;
  n_fish: number;
  unique_requirement_single: boolean;
  unique_requirement_count: number;
  short_chain_count: number;
  long_chain_count: number;
}

export interface WasmSolveOneReturn {
  difficulty: number;
  grid: WasmGrid;
  res: WasmResult<WasmSolveResult, string>;
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

export function solve(input: WasmGrid, useChains: boolean): WasmSolveReturn {
  return mod.solve(input, useChains);
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
