import { useEvent } from "../utils/useEvent.ts";
import { Button } from "./Button.tsx";

export interface ImporterProps {
  onSubmit(grid: string): void;
}

export function Importer(props: ImporterProps) {
  const { onSubmit } = props;

  const onFormSubmit = useEvent((formData: FormData) => {
    onSubmit((formData.get("data") as string) ?? "");
  });

  return (
    <form
      className="bg-light-800 mb-2 flex w-full max-w-[538px] flex-col rounded border p-2 dark:border-blue-400 dark:bg-blue-300"
      action={onFormSubmit}
    >
      <label className="flex flex-col">
        <span className="block dark:text-white">
          Enter the puzzle. You can either paste a link to the 'default' solver or enter a grid of numbers 1-9, letters
          a-i, # and . to denote solutions, blockers, walls and holes.
        </span>
        <textarea className="mt-4 max-w-[90vw] bg-white font-mono tracking-[1em] text-black" rows={10} name="data" />
      </label>
      <Button className="mt-4" type="submit">
        Parse
      </Button>
    </form>
  );
}
