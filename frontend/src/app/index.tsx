import { useEffect, useMemo, useState } from "react";
import * as solver from "../solver/solver.ts";
import { Grid } from "./grid";
import { Header } from "./Header.tsx";
import { useEvent } from "../utils/useEvent.ts";
import { Hint } from "./Hint.tsx";
import { Importer } from "./Importer.tsx";
import { Error } from "./Error.tsx";
import { Generator } from "./Generator.tsx";
import { HistoryNode, SolutionHistory } from "./SolutionHistory.tsx";
import { isOk, WasmOk } from "../solver/wasmTypes.ts";
import { Rating } from "./Rating.tsx";
import { CopyOnClick } from "./CopyOnClick.tsx";

function defaultGrid() {
  const grid = solver.parse([
    "#########",
    "#########",
    "#########",
    "#########",
    "#########",
    "#########",
    "#########",
    "#########",
    "#########",
  ]) as WasmOk<solver.Grid>;
  return grid.Ok;
}

export function App() {
  const [grid, setGrid] = useState(defaultGrid);
  const [gridStr, setGridStr] = useState("");
  const [startingGrid, setStartingGrid] = useState(defaultGrid);
  const [solutionLog, setSolutionLog] = useState<HistoryNode[]>([]);

  const [error, setError] = useState("");

  const [hint, setHint] = useState("");
  useEffect(() => {
    setHint("");
  }, [grid]);

  const [importerOpen, setImporterOpen] = useState(true);
  const [generatorOpen, setGeneratorOpen] = useState(false);
  const [editMode, setEditMode] = useState(false);
  const [focused, _setFocused] = useState<[solver.Grid, solver.Grid] | null>(null);
  const [focusedColors, _setFocusedColors] = useState<number[][][] | null>(null);
  const setFocused = useEvent((before: solver.Grid | null, after: solver.Grid | null, colors: number[][][] | null) => {
    _setFocused(before && after ? [before, after] : null);
    _setFocusedColors(colors);
  });

  const openImporter = useEvent(() => {
    setImporterOpen(true);
    setGeneratorOpen(false);
  });
  const onImport = useEvent((newGrid: string) => {
    const grid = solver.parse(
      newGrid
        .trim()
        .split("\n")
        .map((s) => s.trim()),
    );
    if (isOk(grid)) {
      setImporterOpen(false);
      setGrid(grid.Ok);
      setStartingGrid(grid.Ok);
      setSolutionLog([]);
      setError("");
    } else {
      setError(grid.Err);
    }
  });

  const onGenerate = useEvent((newGrid: solver.Grid, gridStr: string) => {
    setGeneratorOpen(false);
    setGrid(newGrid);
    setStartingGrid(newGrid);
    setSolutionLog([]);
    setGridStr(gridStr);
    setError("");
  });

  const openGenerator = useEvent(() => {
    setImporterOpen(false);
    setGeneratorOpen(true);
  });

  const solveOne = useEvent(() => {
    const { grid: g, res, resDisplay, difficulty } = solver.solve_one(grid);
    setGrid(g);
    setSolutionLog((log) => [...log, { grid: g, difficulty, message: resDisplay, data: res }]);
    if (res && "Err" in resDisplay) {
      setError(resDisplay.Err);
    } else {
      setError("");
    }
  });

  const solve = useEvent((useGuessing: boolean) => {
    const { res } = solver.solve(grid, useGuessing);
    if (!res.length) return;
    setGrid(res[res.length - 1].grid);
    const newLog = res.map((row) => ({
      grid: row.grid,
      difficulty: row.difficulty,
      message: row.resDisplay,
      data: row.res,
    }));
    setSolutionLog((log) => [...log, ...newLog]);
    const last = newLog.length && newLog[newLog.length - 1];
    if (last && "Err" in last.message) {
      setError(last.message.Err);
    } else {
      setError("");
    }
  });

  const onHint = useEvent(() => {
    const { resDisplay } = solver.solve_one(grid);
    if (isOk(resDisplay)) setHint(resDisplay.Ok);
  });

  const isSolved = useMemo(() => grid.cells.every((row) => row.every((cell) => cell.ty !== "Indeterminate")), [grid]);

  return (
    <main className="flex flex-col items-center justify-center p-3 xl:flex-row xl:items-stretch dark:bg-blue-100 dark:text-white">
      <div className="flex">
        <div className="flex flex-col items-center">
          <Header
            isSolved={isSolved}
            onStep={solveOne}
            onHint={onHint}
            onSolve={solve}
            setEditMode={setEditMode}
            editMode={editMode}
            openImporter={openImporter}
            openGenerator={openGenerator}
          />
          <Hint hint={hint} />
          <Error error={error} />
          {importerOpen && <Importer onSubmit={onImport} />}
          {generatorOpen && <Generator onGenerate={onGenerate} />}
          {gridStr && <CopyOnClick data={gridStr}>Copy puzzle to clipboard</CopyOnClick>}
          <Grid
            diffgrid={focused}
            grid={grid}
            setCell={(x, y, cell) => {
              setGrid((grid) => {
                const newRow = [...grid.cells[y]];
                newRow[x] = cell;

                const cells = [...grid.cells];
                cells[y] = newRow;

                return { ...grid, cells };
              });
              setError("");
            }}
            setReqForbs={(ty, idx, val) => {
              setGrid((grid) => {
                const newField = [...grid[ty]];
                newField[idx] = val;
                return { ...grid, [ty]: newField };
              });
              setError("");
            }}
            colors={focusedColors}
          />
          {isSolved && solutionLog.length > 0 && <Rating solutionLog={solutionLog} />}
        </div>
      </div>

      <SolutionHistory startingGrid={startingGrid} solutionLog={solutionLog} onFocus={setFocused} />
    </main>
  );
}
