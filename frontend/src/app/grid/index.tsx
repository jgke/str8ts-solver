import type { Grid, Cell as CellT } from "../../solver/solver.ts";
import { Cell } from "./Cell.tsx";
import { useCallback, useMemo, useState } from "react";
import { useEvent } from "../../utils/useEvent.ts";
import { newDiff, unDiff } from "./diffgrid.ts";

type ReqForbKey = "col_requirements" | "col_forbidden" | "row_requirements" | "row_forbidden";

export interface GridProps {
  grid: Grid;
  diffgrid: [Grid, Grid] | null;

  setCell(x: number, y: number, cell: CellT): void;

  setReqForbs(ty: ReqForbKey, idx: number, val: Set<number>): void;
}

function hasExtras(grid: Grid): boolean {
  return [grid.row_requirements, grid.row_forbidden, grid.col_requirements, grid.col_forbidden].some((sets) =>
    sets.some((set) => set.size),
  );
}

type FocusedTy = null | [number, number] | [ReqForbKey, number];

export function Grid(props: GridProps) {
  const { grid: latestGrid, diffgrid, setCell, setReqForbs } = props;
  const [focused, _setFocused] = useState<FocusedTy>(null);

  const grid = useMemo(
    () => (diffgrid ? newDiff(diffgrid[0], diffgrid[1]) : newDiff(latestGrid, latestGrid)),
    [diffgrid, latestGrid],
  );

  const setFocused = useEvent((val: FocusedTy) => {
    if (!diffgrid) _setFocused(val);
  });
  const showExtras = hasExtras(latestGrid);

  const updateCell = useCallback(
    (x: number, y: number, cell: CellT) => {
      return (set: Set<number>) => {
        setFocused(null);
        let newCell;
        if (cell.ty === "Indeterminate" || cell.ty === "Solution") {
          if (set.size === 1)
            newCell = {
              ty: "Solution",
              num: [...set].toSorted()[0],
            } satisfies CellT;
          else newCell = { ty: "Indeterminate", set } satisfies CellT;
        } else if (cell.ty === "Black" || cell.ty === "Blocker") {
          if (set.size) {
            newCell = { ty: "Blocker", num: [...set].toSorted()[0] } satisfies CellT;
          } else {
            newCell = { ty: "Black" } satisfies CellT;
          }
        } else if (cell.ty === "Requirement") {
          if (set.size === 1)
            newCell = {
              ty: "Requirement",
              num: [...set].toSorted()[0],
            } satisfies CellT;
        }
        if (newCell) setCell(x, y, newCell);
      };
    },
    [setFocused, setCell],
  );

  const updateReqForbs = useCallback(
    (key: ReqForbKey, idx: number) => {
      return (set: Set<number>) => {
        setFocused(null);
        setReqForbs(key, idx, set);
      };
    },
    [setFocused, setReqForbs],
  );

  return (
    <div className="md:bg-light-800 mb-4 flex w-fit flex-col rounded leading-tight text-black md:border md:p-4 md:dark:bg-blue-200">
      {showExtras && (
        <div className="flex w-fit">
          <div className="w-1/12vw h-1/12vw md:h-14 md:w-14" />
          <div className="w-1/12vw h-1/12vw md:h-14 md:w-14" />
          <div className="mr-2" />
          {grid.col_requirements.map((reqs, i) => (
            <Cell
              key={i}
              focused={focused?.[0] === "col_requirements" && focused?.[1] === i}
              cell={{ ty: "Requirements", set: reqs }}
              onBlur={updateReqForbs("col_requirements", i)}
              onFocus={() => setFocused(["col_requirements", i])}
            />
          ))}
        </div>
      )}
      {showExtras && (
        <div className="mb-2 flex w-fit">
          <div className="w-1/12vw h-1/12vw md:h-14 md:w-14" />
          <div className="w-1/12vw h-1/12vw md:h-14 md:w-14" />
          <div className="mr-2" />
          {grid.col_forbidden.map((reqs, i) => (
            <Cell
              key={i}
              focused={focused?.[0] === "col_forbidden" && focused?.[1] === i}
              cell={{ ty: "Blockers", set: reqs }}
              onBlur={updateReqForbs("col_forbidden", i)}
              onFocus={() => setFocused(["col_forbidden", i])}
            />
          ))}
        </div>
      )}
      {grid.cells.map((row, y) => (
        <div key={y} className="flex w-fit">
          {showExtras && (
            <Cell
              focused={focused?.[0] === "row_requirements" && focused?.[1] === y}
              cell={{ ty: "Requirements", set: grid.row_requirements[y] }}
              onBlur={updateReqForbs("row_requirements", y)}
              onFocus={() => setFocused(["row_requirements", y])}
            />
          )}
          {showExtras && (
            <Cell
              focused={focused?.[0] === "row_forbidden" && focused?.[1] === y}
              cell={{ ty: "Blockers", set: grid.row_forbidden[y] }}
              onBlur={updateReqForbs("row_forbidden", y)}
              onFocus={() => setFocused(["row_forbidden", y])}
            />
          )}
          {showExtras && <div className="mr-2" />}
          <div className="flex">
            {row.map((cell, x) => (
              <Cell
                focused={focused?.[0] === x && focused?.[1] === y}
                key={x}
                cell={cell}
                onFocus={() => setFocused([x, y])}
                onBlur={updateCell(x, y, unDiff(cell))}
              />
            ))}
          </div>
        </div>
      ))}
    </div>
  );
}
