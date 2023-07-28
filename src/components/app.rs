use crate::components::grid::RenderGrid;
use crate::components::header::Header;
use crate::components::importer::Importer;
use crate::components::puzzle_rating::PuzzleRating;
use crate::components::solution_history::{solve_result_discriminant, SolutionHistory};
use solver::grid::{Cell, Grid};
use solver::solver::{solve_round, SolveResults};
use std::rc::Rc;
use yew::prelude::*;

pub type HistoryFocusState = (Rc<Vec<usize>>, Option<usize>);
pub type HistoryState = Rc<Vec<(Grid, SolveResults)>>;

fn same_disc_length_at(idx: usize, slice: &[(Grid, SolveResults)]) -> usize {
    let mut final_index = 0;
    let cmp = solve_result_discriminant(0, &(*slice)[idx].1);
    for (_, s) in &(*slice)[idx..] {
        if cmp != solve_result_discriminant(0, s) {
            break;
        }
        final_index += 1;
    }
    final_index - 1
}

fn get_focus_state(
    (history_focus_state, final_idx): (&[usize], Option<usize>),
    grid_state: &Grid,
    history_state: Rc<Vec<(Grid, SolveResults)>>,
) -> (Grid, Option<Grid>) {
    let mut final_index = 0;
    if let Some(i) = history_focus_state.first() {
        final_index = same_disc_length_at(*i, &history_state);
    }
    if final_idx.is_some() {
        final_index = 0;
    }
    match history_focus_state
        .first()
        .map(|a| (a, history_focus_state.get(1)))
    {
        None => ((*grid_state).clone(), None),
        Some((&index, None)) => (
            (*history_state)[index].0.clone(),
            (*history_state)
                .get(index + final_index + 1)
                .map(|(g, _)| g.clone()),
        ),
        Some((&index, Some(&sub_index))) => {
            if let SolveResults::Chain(_, _, list, error_grid) = &(*history_state)[index].1 {
                final_index = same_disc_length_at(sub_index, list);
                if final_idx.is_some() {
                    final_index = 0;
                }
                (
                    list[sub_index].0.clone(),
                    list.get(sub_index + final_index + 1)
                        .map(|(g, _)| g.clone())
                        .or_else(|| Some(error_grid.clone())),
                )
            } else {
                (
                    (*history_state)[index + final_index].0.clone(),
                    (*history_state)
                        .get(index + final_index + 1)
                        .map(|(g, _)| g.clone()),
                )
            }
        }
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let grid_state = use_state(move || {
        let base = vec![
            ".........".to_string(),
            ".........".to_string(),
            ".........".to_string(),
            ".........".to_string(),
            ".........".to_string(),
            ".........".to_string(),
            ".........".to_string(),
            ".........".to_string(),
            ".........".to_string(),
        ];
        Grid::parse(base).unwrap()
    });
    let error_state = use_state(|| None);
    let history_state: UseStateHandle<HistoryState> = use_state(|| Rc::new(Vec::new()));
    let history_focus_state: UseStateHandle<HistoryFocusState> =
        use_state(|| (Rc::new(vec![]), None));
    let history_focus_locked: UseStateHandle<Option<HistoryFocusState>> = use_state(|| None);
    let edit_mode: UseStateHandle<bool> = use_state(|| false);
    let importer_open: UseStateHandle<bool> = use_state(|| true);
    let latest_hint: UseStateHandle<Option<SolveResults>> = use_state(|| None);

    let (focus_state, sub_index) = if let Some(locked) = &*history_focus_locked {
        locked.clone()
    } else {
        (*history_focus_state).clone()
    };

    {
        let latest_hint = latest_hint.clone();
        use_effect_with_deps(move |_| latest_hint.set(None), grid_state.clone());
    }

    let onclick = {
        let grid_state = grid_state.clone();
        let error_state = error_state.clone();
        let history_state = history_state.clone();
        Callback::from(move |_| {
            let mut new_grid = (*grid_state).clone();
            match solve_round(&mut new_grid, true) {
                Ok(str) => {
                    let mut new_history = (**history_state).clone();
                    new_history.push(((*grid_state).clone(), str));
                    history_state.set(Rc::new(new_history));
                }
                Err(e) => {
                    error_state.set(Some(e));
                }
            }
            grid_state.set(new_grid);
        })
    };

    let on_hint = {
        let grid_state = grid_state.clone();
        let error_state = error_state.clone();
        let latest_hint = latest_hint.clone();
        Callback::from(move |_| {
            let mut new_grid = (*grid_state).clone();
            match solve_round(&mut new_grid, true) {
                Ok(str) => {
                    latest_hint.set(Some(str));
                }
                Err(e) => {
                    error_state.set(Some(e));
                }
            }
        })
    };

    let on_focus_change = {
        let history_focus_state = history_focus_state.clone();
        Callback::from(move |e: HistoryFocusState| {
            history_focus_state.set(e);
        })
    };

    let on_focus_click = {
        let history_focus_locked = history_focus_locked.clone();
        let history_focus_state = history_focus_state.clone();
        Callback::from(move |_: ()| {
            if (*history_focus_locked).is_some() {
                history_focus_locked.set(None);
            } else {
                history_focus_locked.set(Some((*history_focus_state).clone()));
            }
        })
    };

    let onclick_all = {
        let grid_state = grid_state.clone();
        let error_state = error_state.clone();
        let history_state = history_state.clone();
        Callback::from(move |_| {
            let mut new_grid = (*grid_state).clone();
            let mut prev_grid = new_grid.clone();
            let mut new_history = (**history_state).clone();
            loop {
                match solve_round(&mut new_grid, true) {
                    Ok(str) => {
                        new_history.push((prev_grid, str.clone()));
                        prev_grid = new_grid.clone();
                        if str == SolveResults::PuzzleSolved {
                            break;
                        }
                    }
                    Err(e) => {
                        error_state.set(Some(e));
                        break;
                    }
                }
            }
            history_state.set(Rc::new(new_history));
            grid_state.set(new_grid);
        })
    };

    let (cur_grid, next_grid) = get_focus_state(
        (&*focus_state, sub_index),
        &grid_state,
        (*history_state).clone(),
    );

    let on_cell_update = {
        let grid_state = grid_state.clone();
        Callback::from(move |((x, y), cell): ((usize, usize), Cell)| {
            let mut new_grid: Grid = (*grid_state).clone();
            new_grid.cells[y][x] = cell;
            grid_state.set(new_grid);
        })
    };

    let set_edit_mode = {
        let edit_mode = edit_mode.clone();
        Callback::from(move |res| {
            edit_mode.set(res);
        })
    };

    let open_importer = {
        let importer_open = importer_open.clone();
        Callback::from(move |_| {
            importer_open.set(true);
        })
    };

    let on_import = {
        let grid_state = grid_state.clone();
        let error_state = error_state.clone();
        let history_state = history_state.clone();
        let history_focus_state = history_focus_state;
        let history_focus_locked = history_focus_locked;
        let edit_mode = edit_mode.clone();
        let importer_open = importer_open.clone();
        Callback::from(move |new_grid| {
            grid_state.set(new_grid);
            error_state.set(None);
            history_state.set(Rc::new(Vec::new()));
            history_focus_state.set((Rc::new(vec![]), None));
            history_focus_locked.set(None);
            edit_mode.set(false);
            importer_open.set(false);
        })
    };

    html! {
        <main class="dark:bg-blue-100 flex flex-col xl:flex-row justify-center items-center xl:items-stretch p-3">
            <div class="flex">
                <div class="flex flex-col items-center">
                    <Header
                        is_solved={grid_state.is_solved()}
                        on_step={onclick}
                        {on_hint}
                        on_solve={onclick_all}
                        edit_mode={*edit_mode}
                        {set_edit_mode}
                        {open_importer} />

                    if *importer_open {
                        <Importer {on_import} />
                    }

                    if let Some(hint) = &*latest_hint {
                        <div class="border dark:bg-blue-400 dark:text-white p-2 my-2">
                            <b class="font-bold">{"Hint:"}</b>
                            { format!("{}", hint) }
                        </div>
                    }

                    <RenderGrid
                        grid={cur_grid}
                        next_grid={next_grid}
                        show_extras={grid_state.has_requirements()}
                        on_change={on_cell_update}
                        edit_mode={*edit_mode}/>
                    if let Some(s) = &*error_state {
                        <div class="mt-4 bg-error p-4 max-w-[600px]">{s}</div>
                    }

                    if grid_state.is_solved() {
                        <PuzzleRating history={(*history_state).clone()} />
                    }
                </div>
            </div>
            <div class="flex flex-col border dark:border-blue-400 w-full md:w-[30rem] p-4 ml-4 md:max-h-[90vh]">
                <h2 class="dark:text-white font-bold text-2xl my-2">{"Solution log"}</h2>
                <div class="overflow-y-scroll">
                    <SolutionHistory
                        history_state={(*history_state).clone()}
                        focus_state={(focus_state, sub_index)}
                        focus_chain={vec![]}
                        {on_focus_change}
                        {on_focus_click}
                        nested={false} />
                </div>
            </div>
        </main>
    }
}
