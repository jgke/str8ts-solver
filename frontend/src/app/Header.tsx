import { ChangeEvent, useState } from "react";
import { Button, LabelButton } from "./Button.tsx";
import { useEvent } from "../utils/useEvent.ts";

export interface HeaderProps {
  isSolved: boolean;

  onStep(): void;

  onHint(): void;

  onSolve(useGuessing: boolean): void;

  setEditMode(mode: boolean): void;

  editMode: boolean;

  openImporter(): void;

  openGenerator(): void;

  onCopy(): void;
}

export function Header(props: HeaderProps) {
  const { isSolved, onStep, onHint, onSolve, setEditMode, editMode, openImporter, openGenerator, onCopy } = props;

  const [hamburger, setHamburger] = useState(false);

  const onEditMode = useEvent((e: ChangeEvent<HTMLInputElement>) => setEditMode(e.target.id === "enable-edit"));
  const openHamburger = useEvent(() => setHamburger(true));
  const closeHamburger = useEvent(() => setHamburger(false));

  const onImporter = useEvent(() => {
    closeHamburger();
    openImporter();
  });

  const onGenerator = useEvent(() => {
    closeHamburger();
    openGenerator();
  });

  const onSolveWithoutGuessingClick = useEvent(() => {
    closeHamburger();
    onSolve(false);
  });

  const onSolveClick = useEvent(() => {
    closeHamburger();
    onSolve(true);
  });

  const onCopyClick = useEvent(() => {
    closeHamburger();
    onCopy();
  });

  const label_classNamees =
    "my-2 mr-2 rounded inline-block peer-checked:font-bold w-32 flex justify-center cursor-pointer";
  const spanClass =
    "block parent-sibling-checked:border-y-4 parent-sibling-checked:border-y-blue-800 dark:parent-sibling-checked:border-y-blue-700 parent-sibling-checked:py-1 py-2 px-1 w-full text-center";

  return (
    <div className="mb-2 flex w-full flex-wrap items-center">
      <div className="inline-block">
        <input id="disable-edit" className="peer hidden" type="radio" onChange={onEditMode} checked={!editMode} />
        <LabelButton htmlFor="disable-edit" className={label_classNamees}>
          <span className={spanClass}>Edit numbers</span>
        </LabelButton>
      </div>
      <div className="inline-block">
        <input id="enable-edit" className="peer hidden" type="radio" onChange={onEditMode} checked={editMode} />
        <LabelButton htmlFor="enable-edit" className={label_classNamees}>
          <span className={spanClass}>Toggle color</span>
        </LabelButton>
      </div>
      <div className="flex-grow" />
      <Button disabled={isSolved} onClick={onStep}>
        Solve 1 step
      </Button>
      <Button disabled={isSolved} onClick={onHint}>
        Show hint
      </Button>
      <input
        id="hamburger"
        className="peer/hamburger hidden"
        type="checkbox"
        onChange={openHamburger}
        checked={hamburger}
      />
      <LabelButton htmlFor="hamburger" className="h-[42px] w-[42px] cursor-pointer p-2">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          fill="none"
          viewBox="0 0 24 24"
          strokeWidth={1.5}
          stroke="currentColor"
          className="h-6 w-6"
        >
          <path strokeLinecap="round" strokeLinejoin="round" d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5" />
        </svg>
        <span className="sr-only">Open menu</span>
      </LabelButton>
      {hamburger && <div className="fixed top-0 right-0 bottom-0 left-0 cursor-pointer" onClick={closeHamburger}></div>}
      {hamburger && (
        <div>
          <div className="bg-light-700 absolute flex flex-col rounded border p-4 dark:border-blue-300 dark:bg-blue-200">
            <Button className="mb-2" onClick={onImporter}>
              Import puzzle
            </Button>
            <Button className="mb-2" onClick={onGenerator}>
              Generate puzzle
            </Button>
            <Button className="mb-2" onClick={onCopyClick}>
              Copy puzzle to clipboard
            </Button>
            <Button disabled={isSolved} className="mb-2" onClick={onSolveWithoutGuessingClick}>
              Solve without guessing
            </Button>
            <Button disabled={isSolved} onClick={onSolveClick}>
              Solve
            </Button>
          </div>
        </div>
      )}
    </div>
  );
}
