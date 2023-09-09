use solver::difficulty::{puzzle_difficulty, Difficulty};
use solver::grid::Grid;
use solver::solver::SolveResults;
use std::rc::Rc;
use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct PuzzleRatingProps {
    pub history: Rc<Vec<(Grid, SolveResults)>>,
}

#[function_component(PuzzleRating)]
pub fn render_puzzle_rating(props: &PuzzleRatingProps) -> Html {
    let Difficulty {
        star_count,
        move_count: _,
        basic_reductions,
        min_max_reductions,
        cross_compartment_ranges,
        sets,
        maintain_reqs_and_blocks,
        setti,
        x_wing,
        swordfish,
        medusa,
        n_fish,
        short_chain_count,
        long_chain_count,
    } = puzzle_difficulty(&props.history.iter().map(|(_, s)| s).collect::<Vec<_>>());

    let star = html! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" className="w-6 h-6">
          <path fillRule="evenodd" d="M10.788 3.21c.448-1.077 1.976-1.077 2.424 0l2.082 5.007 5.404.433c1.164.093 1.636 1.545.749 2.305l-4.117 3.527 1.257 5.273c.271 1.136-.964 2.033-1.96 1.425L12 18.354 7.373 21.18c-.996.608-2.231-.29-1.96-1.425l1.257-5.273-4.117-3.527c-.887-.76-.415-2.212.749-2.305l5.404-.433 2.082-5.006z" clipRule="evenodd" />
        </svg>
    };

    let disabled_class = "hidden";
    let enabled_class =
        "dark:bg-blue-300 rounded border border-blue-400 dark:text-white p-2 my-2 w-full md:w-2/5 mr-2 font-bold";
    let enabled_hard_class =
        "font-bold dark:bg-blue-500 rounded border dark:border-blue-600 dark:text-white p-2 my-2 w-full md:w-2/5 mr-2";
    let enabled_very_hard_class = "font-bold dark:bg-blue-600 rounded border dark:border-bg-blue-600 dark:text-white p-2 my-2 w-full md:w-2/5 mr-2";
    let hidden_class = "hidden";

    html! {
        <div class="flex flex-col">
            <h2 class="dark:text-white font-bold text-2xl my-2">{"Puzzle difficulty rating"}</h2>
            <div class="flex h-16 text-star">
                 { (1..=star_count).map(|_| star.clone()).collect::<Html>() }
            </div>
            <h2 class="dark:text-white font-bold text-xl my-2">{"Required tactics"}</h2>

            <div class="flex flex-wrap md:w-[600px]">
                <span class={if basic_reductions { enabled_class } else { disabled_class }}>{"Basic reductions"}</span>
                <span class={if min_max_reductions { enabled_class } else { disabled_class }}>{"Intra-compartment range checks"}</span>
                <span class={if cross_compartment_ranges { enabled_class } else { disabled_class }}>{"Cross-compartment range checks"}</span>
                <span class={if sets { enabled_class } else { disabled_class }}>{"Naked and hidden sets"}</span>
                <span class={if maintain_reqs_and_blocks { enabled_hard_class } else { hidden_class }}>{"Maintain lists of required and forbidden numbers "}</span>
                <span class={if setti { enabled_hard_class } else { hidden_class }}>{"Setti"}</span>
                <span class={if x_wing { enabled_hard_class } else { hidden_class }}>{"X-wing"}</span>
                <span class={if swordfish { enabled_hard_class } else { hidden_class }}>{"Swordfish"}</span>
                <span class={if medusa { enabled_hard_class } else { hidden_class }}>{"Medusa"}</span>
                <span class={if n_fish > 4 { enabled_hard_class } else { hidden_class }}>{format!("{}-fish", n_fish)}</span>
                <span class={if short_chain_count > 0 { enabled_very_hard_class } else { hidden_class }}>{format!("Short chain x {}", short_chain_count)}</span>
                <span class={if long_chain_count > 0 { enabled_very_hard_class } else { hidden_class }}>{format!("Long chain x {}", long_chain_count)}</span>
            </div>
        </div>
    }
}
