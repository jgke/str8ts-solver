import { getColors, Grid, gridFromWasm } from "../solver/solver.ts";
import { isOk, WasmResult, WasmSolveResult, WasmSolveType, WasmValidationResult } from "../solver/wasmTypes.ts";
import { unreachable } from "../utils/unreachable.ts";
import { MouseEventHandler, MouseEvent, useMemo, useState } from "react";
import { useEvent } from "../utils/useEvent.ts";

export interface HistoryNode {
  grid: Grid;
  message: WasmResult<string, string>;
  data: WasmResult<WasmSolveResult, WasmValidationResult>;
  difficulty: number;
}

export interface HistoryGroup {
  grid: Grid;
  message: WasmResult<string, string>;
  data: WasmResult<WasmSolveResult, WasmValidationResult>;
  children: HistoryNode[];
}

export interface SolutionHistoryProps {
  startingGrid: Grid;
  solutionLog: HistoryNode[];

  onFocus(before: Grid | null, after: Grid | null, colors: number[][][] | null): void;
}

interface FocusProps {
  prev: Grid;
  path: string;
  focused: string | null;
  manualFocus: boolean;

  onFocus(id: null): void;

  onFocus(id: string, prev: Grid, next: Grid, colors: number[][][] | null): void;

  setManualFocus(focus: boolean): void;
}

function borderForSolution(cell: WasmSolveType): string {
  if (
    cell === "PuzzleSolved" ||
    cell === "OutOfBasicStrats" ||
    cell === "UpdateImpossibles" ||
    cell === "Singles" ||
    cell === "Stranded" ||
    cell === "DefiniteMinMax" ||
    cell === "RequiredRange" ||
    cell === "EnumerateSolutions"
  )
    return "";
  if (cell === "RequiredAndForbidden" || cell === "RowColBrute" || "Setti" in cell)
    return "border-t-8 border-t-blue-700";
  if ("YWing" in cell || "Sets" in cell) return "border-t-8 border-t-blue-700";
  if ("Fish" in cell && (cell.Fish === 2 || cell.Fish === 3)) return "border-t-8 border-t-blue-700";
  if ("Fish" in cell) return "border-t-8 border-t-blue-800";
  if ("SimpleUniqueRequirement" in cell) return "border-t-8 border-t-blue-800";
  if ("Medusa" in cell || "UniqueRequirement" in cell) return "border-t-8 border-t-blue-800";
  if ("StartGuess" in cell || "GuessStep" in cell || "EndGuess" in cell) return "border-t-8 border-t-blue-800";
  unreachable(cell);
}

const liClass = "border dark:border-blue-400 mt-2 mr-2 dark:text-white rounded";

interface SolutionInnerLogItemProps extends FocusProps {
  num: number;
  node: HistoryNode;
}

function useFocusProps(
  grid: Grid,
  colors: number[][][] | null,
  focusProps: FocusProps,
): [boolean, MouseEventHandler, MouseEventHandler] {
  const { path, focused, manualFocus, setManualFocus, onFocus, prev } = focusProps;

  const onHover = useEvent((e: MouseEvent) => {
    e.preventDefault();
    e.stopPropagation();
    if (!manualFocus) {
      onFocus(path, prev, grid, colors);
    }
  });

  const onClick = useEvent((e: MouseEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setManualFocus(!manualFocus);
    onFocus(path, prev, grid, colors);
  });

  return [focused === path, onHover, onClick];
}

function SolutionInnerLogItem(props: SolutionInnerLogItemProps) {
  const { num, node, ...focusProps } = props;
  const [focused, onHover, onClick] = useFocusProps(node.grid, getColors(node.data), focusProps);

  const extraClass = focused ? "font-bold dark:bg-blue-400" : "font-medium bg-light-900 dark:bg-blue-100";

  return (
    <li
      className={`inline-block h-10 w-10 items-center justify-center ${liClass} ${extraClass}`}
      onMouseOver={onHover}
      onClick={onClick}
    >
      <div className="w-auto cursor-pointer rounded p-2 text-center">{num}</div>
    </li>
  );
}

interface SolutionLogListItemProps extends FocusProps {
  nested: boolean;
  row: HistoryGroup;
}

function SolutionLogListItem(props: SolutionLogListItemProps) {
  const { nested, row, ...focusProps } = props;
  const [focused, onHover, onClick] = useFocusProps(row.grid, getColors(row.data), focusProps);
  const extraClass = focused
    ? nested
      ? "font-bold dark:bg-blue-400"
      : "font-bold dark:bg-blue-600"
    : nested
      ? "font-medium bg-light-900 dark:bg-blue-500"
      : "font-medium bg-light-900 dark:bg-blue-100";

  let nestedGuessStep: HistoryGroup[] | undefined;
  if (isOk(row.data) && typeof row.data.Ok.ty === "object" && "GuessStep" in row.data.Ok.ty) {
    nestedGuessStep = row.data.Ok.ty.GuessStep[2].map(([grid, result, msg]) => ({
      grid: gridFromWasm(grid),
      data: { Ok: result },
      message: { Ok: msg },
      children: [],
    }));
  }

  return (
    <li className={`${liClass} ${extraClass}`} onMouseOver={onHover} onClick={onClick}>
      <div className={`${isOk(row.data) ? borderForSolution(row.data.Ok.ty) : ""} cursor-pointer rounded p-2`}>
        {isOk(row.message) ? row.message.Ok : row.message.Err}
        {nestedGuessStep && (
          <SolutionLogList
            {...focusProps}
            solutionLog={nestedGuessStep}
            nested={true}
            prev={focusProps.prev}
            path={`${focusProps.path}_`}
          />
        )}
        {row.children.length > 1 && (
          <ul>
            {row.children.map((child, i) => (
              <SolutionInnerLogItem
                {...focusProps}
                key={i}
                num={i + 1}
                node={child}
                prev={i === 0 ? focusProps.prev : row.children[i - 1].grid}
                path={`${focusProps.path}.${i}`}
              />
            ))}
          </ul>
        )}
      </div>
    </li>
  );
}

interface SolutionLogListProps extends FocusProps {
  nested: boolean;
  solutionLog: HistoryGroup[];
}

function SolutionLogList(props: SolutionLogListProps) {
  const { nested, solutionLog, ...focusProps } = props;

  return (
    <ul>
      {solutionLog.map((row, i) => (
        <SolutionLogListItem
          {...focusProps}
          key={i}
          nested={nested}
          row={row}
          prev={i === 0 ? focusProps.prev : solutionLog[i - 1].grid}
          path={`${focusProps.path}.${i}`}
        />
      ))}
    </ul>
  );
}

function groupNodes(nodes: HistoryNode[]): HistoryGroup[] {
  if (!nodes.length) return [];
  const res: HistoryGroup[] = [];
  let cur: HistoryGroup = {
    ...nodes[0],
    children: [nodes[0]],
  };
  for (const node of nodes.slice(1)) {
    if ("Ok" in cur.message && "Ok" in node.message && cur.message.Ok === node.message.Ok) {
      cur.children.push(node);
      cur.grid = node.grid;
    } else {
      res.push(cur);
      cur = { ...node, children: [node] };
    }
  }
  res.push(cur);
  return res;
}

export function SolutionHistory(props: SolutionHistoryProps) {
  const { startingGrid, solutionLog, onFocus } = props;
  const grouped = useMemo(() => groupNodes(solutionLog), [solutionLog]);
  const [manualFocus, setManualFocus] = useState(false);
  const [focused, setFocused] = useState<string | null>(null);

  const onFocusCb = useEvent((id: string | null, grid?: Grid, other?: Grid, colors?: number[][][] | null) => {
    setFocused(id);
    onFocus(grid ?? null, other ?? null, colors ?? null);
  });

  const onFocusExit = useEvent((e: MouseEvent) => {
    e.preventDefault();
    e.stopPropagation();
    if (!manualFocus) onFocusCb(null);
  });

  return (
    <div
      className="mx-4 flex w-full flex-col border p-4 md:max-h-[90vh] md:w-[30rem] dark:border-blue-400"
      onMouseLeave={onFocusExit}
    >
      <h2 className="my-2 text-2xl font-bold dark:text-white">Solution log</h2>
      <div className="cursor-pointer overflow-y-scroll">
        <SolutionLogList
          prev={startingGrid}
          solutionLog={grouped}
          nested={false}
          path=""
          setManualFocus={setManualFocus}
          manualFocus={manualFocus}
          focused={focused}
          onFocus={onFocusCb}
        />
      </div>
    </div>
  );
}
