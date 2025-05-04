import { HistoryNode } from "./SolutionHistory.tsx";
import { useMemo } from "react";
import { puzzleDifficulty } from "../solver/solver.ts";
import { isOk } from "../solver/wasmTypes.ts";

export interface RatingProps {
  solutionLog: HistoryNode[];
}

function Star() {
  /* https://heroicons.com/solid MIT */
  return (
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
      <path
        fillRule="evenodd"
        d="M10.788 3.21c.448-1.077 1.976-1.077 2.424 0l2.082 5.007 5.404.433c1.164.093 1.636 1.545.749 2.305l-4.117 3.527 1.257 5.273c.271 1.136-.964 2.033-1.96 1.425L12 18.354 7.373 21.18c-.996.608-2.231-.29-1.96-1.425l1.257-5.273-4.117-3.527c-.887-.76-.415-2.212.749-2.305l5.404-.433 2.082-5.006z"
        clipRule="evenodd"
      />
    </svg>
  );
}

export function Rating(props: RatingProps) {
  const { solutionLog } = props;
  const difficulty = useMemo(
    () =>
      puzzleDifficulty(
        solutionLog
          .map((node) => node.data)
          .filter(isOk)
          .map((node) => node.Ok),
      ),
    [solutionLog],
  );

  const {
    star_count,
    basic_reductions,
    min_max_reductions,
    cross_compartment_ranges,
    maintain_reqs_and_blocks,
    sets,
    setti,
    x_wing,
    swordfish,
    medusa,
    n_fish,
    unique_requirement_single,
    unique_requirement_count,
    short_chain_count,
    long_chain_count,
  } = difficulty;

  const disabled_class = "hidden";
  const enabled_class =
    "dark:bg-blue-300 rounded border border-blue-400 dark:text-white p-2 my-2 w-full md:w-2/5 mr-2 font-bold";
  const enabled_hard_class =
    "font-bold dark:bg-blue-500 rounded border dark:border-blue-600 dark:text-white p-2 my-2 w-full md:w-2/5 mr-2";
  const enabled_very_hard_class =
    "font-bold dark:bg-blue-600 rounded border dark:border-bg-blue-600 dark:text-white p-2 my-2 w-full md:w-2/5 mr-2";
  const hidden_class = "hidden";

  return (
    <div className="flex flex-col">
      <h2 className="my-2 text-2xl font-bold dark:text-white">Puzzle difficulty rating</h2>
      <div id="puzzle-rating-stars" className="text-star flex h-16">
        {Array.from({ length: star_count }, (_, i) => (
          <Star key={i} />
        ))}
      </div>
      <h2 className="my-2 text-xl font-bold dark:text-white">Required tactics</h2>

      <div className="flex flex-wrap md:w-[600px]">
        <span className={basic_reductions ? enabled_class : disabled_class}>Basic reductions</span>
        <span className={min_max_reductions ? enabled_class : disabled_class}>Intra-compartment range checks</span>
        <span className={cross_compartment_ranges ? enabled_class : disabled_class}>
          Cross-compartment range checks
        </span>
        <span className={sets ? enabled_class : disabled_class}>Naked and hidden sets</span>
        <span className={maintain_reqs_and_blocks ? enabled_hard_class : hidden_class}>
          Maintain lists of required and forbidden numbers
        </span>
        <span className={setti ? enabled_hard_class : hidden_class}>Setti</span>
        <span className={x_wing ? enabled_hard_class : hidden_class}>X-wing</span>
        <span className={swordfish ? enabled_hard_class : hidden_class}>Swordfish</span>
        <span className={medusa ? enabled_hard_class : hidden_class}>Medusa</span>
        <span className={n_fish > 4 ? enabled_hard_class : hidden_class}>{n_fish}-fish</span>
        <span className={unique_requirement_single ? enabled_very_hard_class : hidden_class}>Unique requirement</span>
        <span className={unique_requirement_count > 0 ? enabled_very_hard_class : hidden_class}>
          Unique solution constraint x {unique_requirement_count}
        </span>
        <span className={short_chain_count > 0 ? enabled_very_hard_class : hidden_class}>
          Short chain x {short_chain_count}
        </span>
        <span className={long_chain_count > 0 ? enabled_very_hard_class : hidden_class}>
          Long chain x {long_chain_count}
        </span>
      </div>
    </div>
  );
}
