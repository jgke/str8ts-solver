import { useEffect, useRef } from "react";
import { DiffCell } from "./diffgrid.ts";

export interface IndeterminateCellContentProps {
  set: [Set<number>, Set<number>];
  colors: number[] | null;
}

function IndeterminateCellContent(props: IndeterminateCellContentProps) {
  const {
    set: [left, right],
    colors,
  } = props;
  return [0, 1, 2].map((y) => (
    <div key={y} className="flex w-full items-center justify-evenly">
      {[0, 1, 2].map((x) => {
        const num = y * 3 + x + 1;
        const content = left.has(num) || right.has(num) ? `${num}` : "";
        const color = colors?.[num];
        const bg = color === 1 ? "bg-primary-light" : color === 2 ? "bg-blue-800" : "bg-error";

        if (left.has(num) != right.has(num))
          return (
            <span key={x} className="h-4 w-4 bg-blue-300 text-center text-white">
              {content}
            </span>
          );
        if (color)
          return (
            <span key={x} className={`h-4 w-4 ${bg} text-center text-black`}>
              {content}
            </span>
          );
        return (
          <span key={x} className="h-4 w-4 text-center">
            {content}
          </span>
        );
      })}
    </div>
  ));
}

export interface CellContentProps {
  cell:
    | DiffCell
    | { ty: "Requirements"; set: [Set<number>, Set<number>] }
    | {
        ty: "Blockers";
        set: [Set<number>, Set<number>];
      };
  colors: number[] | null;
}

const emptySign = "\u{2205}";

export function CellContent(props: CellContentProps) {
  const { colors, cell } = props;
  const content_base = "flex items-center justify-center w-full h-full font-bold text-[6vw] md:text-3xl";

  switch (cell.ty) {
    case "Requirement":
      return <span className={`${content_base} bg-white`}>{cell.num}</span>;
    case "Blocker":
      return <span className={`${content_base} bg-primary-black text-white`}>{cell.num}</span>;
    case "Solution":
      return <span className={`${content_base} bg-white text-blue-600 italic`}>{cell.num}</span>;
    case "Indeterminate":
      return cell.set[0].size === 0 ? (
        <span className={`${content_base} bg-error`}>{emptySign}</span>
      ) : (
        <div className="flex h-full w-full flex-col items-center justify-evenly bg-white p-1 text-[2vw] md:text-xs">
          <IndeterminateCellContent set={cell.set} colors={colors} />
        </div>
      );
    case "Requirements":
      return (
        <div className="bg-primary-light dark:bg-primary flex h-full w-full flex-col justify-evenly p-1 text-[2vw] md:text-xs dark:text-white">
          <IndeterminateCellContent set={cell.set} colors={colors} />
        </div>
      );
    case "Blockers":
      return (
        <div className="bg-primary-black flex h-full w-full flex-col justify-evenly p-1 text-[2vw] text-white md:text-xs dark:bg-black dark:text-white">
          <IndeterminateCellContent set={cell.set} colors={colors} />
        </div>
      );
    case "Black":
      return <span className={`${content_base} bg-primary-black text-white`}></span>;
  }
}

export interface CellProps {
  cell:
    | DiffCell
    | { ty: "Requirements"; set: [Set<number>, Set<number>] }
    | {
        ty: "Blockers";
        set: [Set<number>, Set<number>];
      };
  sidebarShown?: boolean;
  focused: boolean;
  colors: number[] | null;

  onFocus(): void;

  onBlur(numbers: Set<number>): void;
}

export function Cell(props: CellProps) {
  const { cell, colors } = props;
  const cell_size_small = "w-1/12vw h-1/12vw md:w-14 md:h-14";
  const cell_size_large = "w-1/10vw h-1/10vw md:w-14 md:h-14";
  const cell_size = props.sidebarShown ? cell_size_small : cell_size_large;

  const textAreaRef = useRef<HTMLTextAreaElement | null>(null);

  useEffect(() => {
    if (textAreaRef.current) textAreaRef.current.select();
  });

  const defaultValue =
    cell.ty === "Black"
      ? ""
      : cell.ty === "Indeterminate" || cell.ty === "Requirements" || cell.ty === "Blockers"
        ? [...cell.set[0]].toSorted().join("")
        : `${cell.num}`;

  if (props.focused) {
    return (
      <div className={`border-primary border ${cell_size}`}>
        <textarea
          defaultValue={defaultValue}
          ref={textAreaRef}
          className={`resize-none border-none bg-white text-black ${cell_size}`}
          autoFocus
          onBlur={(e) =>
            props.onBlur(
              new Set(
                [...e.target.value]
                  .map((num) => Number.parseInt(num, 10))
                  .filter((num) => Number.isInteger(num) && num > 0 && num <= 9),
              ),
            )
          }
        />
      </div>
    );
  }

  return (
    <div className={`border-primary border dark:border-blue-300 ${cell_size}`} onClick={props.onFocus}>
      <CellContent cell={props.cell} colors={colors} />
    </div>
  );
}
