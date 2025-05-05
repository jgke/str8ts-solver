import { useEvent } from "../utils/useEvent.ts";
import { useFormStatus } from "react-dom";
import { Grid, gridFromWasm } from "../solver/solver.ts";
import { generate } from "../solver/wasmTypes.ts";

export interface GeneratorProps {
  onGenerate(grid: Grid, gridStr: string): void;
}

function GeneratorFields() {
  const { pending } = useFormStatus();
  return (
    <>
      <label className="flex items-center py-2">
        <span className="mr-4 block dark:text-white">Puzzle size</span>
        <input className="max-w-8 bg-white p-1 text-black" name="size" defaultValue="9" />
      </label>
      <label className="flex items-center py-2">
        <span className="mr-4 block dark:text-white">Empty black cell count</span>
        <input className="max-w-8 bg-white p-1 text-black" name="blocker_count" defaultValue="10" />
      </label>
      <label className="flex items-center py-2">
        <span className="mr-4 block dark:text-white">Black number count</span>
        <input className="max-w-8 bg-white p-1 text-black" name="blocker_num_count" defaultValue="5" />
      </label>
      <label className="flex items-center py-2">
        <span className="mr-4 block dark:text-white">Target difficulty</span>
        <select className="bg-white p-1 text-black" name="difficulty" defaultValue="4">
          <option value="1">Trivial</option>
          <option value="2">Easy</option>
          <option value="3">Medium</option>
          <option value="4">Hard</option>
          <option value="5">(SLOW) Settis, small fishes</option>
          <option value="6">(VERY SLOW) Large fishes, simple guesses</option>
          <option value="7">(EXTREMELY SLOW) Complex guesses</option>
        </select>
      </label>
      <label className="flex items-center py-2">
        <span className="block dark:text-white">Symmetric</span>
        <input type="checkbox" className="ml-4 text-black" name="symmetric" defaultChecked />
      </label>
      <button
        className="bg-light-800 disabled:text-light-300 dark:disabled:text-light-300 mt-4 flex-grow-0 rounded border p-2 font-bold text-black disabled:border-transparent dark:border-blue-400 dark:bg-blue-300 dark:text-white"
        type="submit"
        disabled={pending}
      >
        Parse
      </button>
      {pending && <p>Calculating...</p>}
    </>
  );
}

export function Generator(props: GeneratorProps) {
  const { onGenerate } = props;
  const onSubmit = useEvent(async (data: FormData) => {
    const size = Number.parseInt(data.get("size") as string, 10);
    const blockerCount = Number.parseInt(data.get("blocker_count") as string, 10);
    const blockerNumCount = Number.parseInt(data.get("blocker_num_count") as string, 10);
    const difficulty = Number.parseInt(data.get("difficulty") as string, 10);
    const symmetric = (data.get("symmetric") as string) === "on";
    try {
      const { grid, grid_str } = await generate({
        size,
        blocker_count: blockerCount,
        blocker_num_count: blockerNumCount,
        symmetric: symmetric,
        target_difficulty: difficulty,
      });
      onGenerate(gridFromWasm(grid), grid_str);
    } catch (error) {
      console.error(error);
    }
  });
  return (
    <form
      className="bg-light-800 mb-2 flex w-full max-w-[538px] flex-col rounded border p-2 dark:border-blue-400 dark:bg-blue-300"
      action={onSubmit}
    >
      <GeneratorFields />
    </form>
  );
}
