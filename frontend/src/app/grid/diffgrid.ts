import { Cell, Grid } from "../../solver/solver.ts";
import { zip } from "../../utils/zip.ts";
import { unreachable } from "../../utils/unreachable.ts";

export type DiffCell =
  | { ty: "Requirement"; num: number }
  | { ty: "Solution"; num: number }
  | { ty: "Blocker"; num: number }
  | { ty: "Indeterminate"; set: [Set<number>, Set<number>] }
  | { ty: "Black" };

export interface DiffGrid {
  cells: DiffCell[][];
  row_requirements: [Set<number>, Set<number>][];
  col_requirements: [Set<number>, Set<number>][];
  row_forbidden: [Set<number>, Set<number>][];
  col_forbidden: [Set<number>, Set<number>][];
}

function toSet(cell: Cell): Set<number> {
  switch (cell.ty) {
    case "Black":
      return new Set();
    case "Requirement":
      return new Set([cell.num]);
    case "Solution":
      return new Set([cell.num]);
    case "Blocker":
      return new Set([cell.num]);
    case "Indeterminate":
      return new Set(cell.set);
    default:
      unreachable(cell);
  }
}

export function newDiff(left: Grid, right: Grid): DiffGrid {
  const cells: DiffCell[][] = left.cells.map((lrow, y) =>
    lrow.map((lcell, x) => {
      const rcell = right.cells[y][x];
      if (lcell.ty === "Indeterminate" || rcell.ty === "Indeterminate")
        return {
          ty: "Indeterminate",
          set: [toSet(lcell), toSet(rcell)],
        };
      if (lcell.ty === "Black" || rcell.ty === "Black") return { ty: "Black" };
      return { ty: lcell.ty, num: lcell.num };
    }),
  );
  const row_requirements = zip(left.row_requirements, right.row_requirements);
  const col_requirements = zip(left.col_requirements, right.col_requirements);
  const row_forbidden = zip(left.row_forbidden, right.row_forbidden);
  const col_forbidden = zip(left.col_forbidden, right.col_forbidden);

  return { cells, row_requirements, col_requirements, row_forbidden, col_forbidden };
}

export function unDiff(cell: DiffCell): Cell {
  if (cell.ty === "Indeterminate") return { ty: "Indeterminate", set: cell.set[0] };
  return cell;
}
