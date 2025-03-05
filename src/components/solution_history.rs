use crate::components::app::HistoryFocusState;
use itertools::Itertools;
use solver::grid::Grid;
use solver::solver::{SolveResults, ValidationResult};
use solver::strats::UrResult;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlElement, MouseEvent};
use yew::{classes, function_component, html, Callback, Html, Properties};

fn border_for_solution(cell: &SolveResults) -> &'static str {
    match cell {
        SolveResults::PuzzleSolved
        | SolveResults::OutOfBasicStrats
        | SolveResults::UpdateImpossibles
        | SolveResults::Singles
        | SolveResults::Stranded
        | SolveResults::DefiniteMinMax
        | SolveResults::RequiredRange
        | SolveResults::Sets(_) => "",
        SolveResults::RequiredAndForbidden | SolveResults::RowColBrute | SolveResults::Setti(_) => {
            "border-t-8 border-t-blue-700"
        }
        SolveResults::Fish(2 | 3) => "border-t-8 border-t-blue-700",
        SolveResults::Fish(_) => "border-t-8 border-t-blue-800",
        SolveResults::SimpleUniqueRequirement(..) => "border-t-8 border-t-blue-800",
        SolveResults::UniqueRequirement(..) => "border-t-8 border-t-blue-800",
        SolveResults::StartChain(_, _)
        | SolveResults::Chain(_, _, _, _)
        | SolveResults::EndChain(_) => "border-t-8 border-t-blue-800",
    }
}

#[derive(PartialEq, Properties)]
pub struct SolutionHistoryProps {
    pub focus_chain: Vec<usize>,
    pub history_state: Rc<Vec<(Grid, SolveResults)>>,
    pub focus_state: HistoryFocusState,
    pub on_focus_change: Callback<HistoryFocusState>,
    pub on_focus_click: Callback<()>,
    pub nested: bool,
}

pub fn solve_result_discriminant(index: usize, res: &SolveResults) -> usize {
    match res {
        SolveResults::UpdateImpossibles => 0,
        SolveResults::Singles => 1,
        SolveResults::Stranded => 2,
        SolveResults::DefiniteMinMax => 3,
        SolveResults::RequiredRange => 4,
        SolveResults::Sets(_) => 5,
        SolveResults::RequiredAndForbidden => 6,
        SolveResults::RowColBrute => 7,
        SolveResults::Setti(_) => 8,
        SolveResults::Fish(_) => 9,
        SolveResults::SimpleUniqueRequirement(UrResult::SingleUnique(..)) => 10_000 + index,
        SolveResults::SimpleUniqueRequirement(UrResult::IntraCompartmentUnique(..)) => 20_000 + index,
        SolveResults::SimpleUniqueRequirement(UrResult::ClosedSetCompartment(..)) => 30_000 + index,
        SolveResults::SimpleUniqueRequirement(UrResult::SingleCellWouldBecomeFree(..)) => 40_000 + index,
        SolveResults::SimpleUniqueRequirement(UrResult::UrSetti(..)) => 50_000 + index,
        SolveResults::SimpleUniqueRequirement(UrResult::SolutionCausesClosedSets(..)) => 60_000 + index,
        SolveResults::UniqueRequirement(..) => 11,
        SolveResults::StartChain(_, _) => 1_000_000 + index,
        SolveResults::Chain(_, _, _, _) => 2_000_000 + index,
        SolveResults::EndChain(_) => 3_000_000 + index,
        SolveResults::PuzzleSolved => 12,
        SolveResults::OutOfBasicStrats => 13,
    }
}

fn get_hl_list(s: &SolveResults) -> String {
    match s {
        SolveResults::EndChain(ValidationResult::Ambiguous { cells }) => format!("{:?}", cells),
        _ => "".to_string(),
    }
}
fn parse_hl(s: String) -> Option<Vec<(usize, usize)>> {
    let content = s.split("[").skip(1).next()?.split("]").next()?;
    Some(
        content
            .split("),")
            .filter_map(|pair| {
                let nums = pair
                    .trim()
                    .split("(")
                    .skip(1)
                    .next()?
                    .split(", ")
                    .filter_map(|s| s.trim_end_matches(')').parse().ok())
                    .collect::<Vec<usize>>();
                Some((*nums.get(0)?, *nums.get(1)?))
            })
            .collect(),
    )
}

#[function_component]
pub fn SolutionHistory(props: &SolutionHistoryProps) -> Html {
    let on_mouse = {
        let on_focus_change = props.on_focus_change.clone();
        let focus_chain = props.focus_chain.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            e.stop_propagation();
            let val: Option<(usize, Option<usize>, Option<Vec<(usize, usize)>>)> = (|| {
                let target: EventTarget = e.target()?;
                let elem = target.dyn_into::<HtmlElement>().ok()?;
                let val: String = elem.get_attribute("data-id")?;
                let val2: Option<usize> = elem
                    .get_attribute("data-sub-id")
                    .and_then(|a| a.parse::<usize>().ok());
                let hl_list = elem.get_attribute("data-hl-list").and_then(|s| parse_hl(s));
                Some((val.parse::<usize>().ok()?, val2, hl_list))
            })();
            if let Some((val, sub_val, hl_list)) = val {
                on_focus_change.emit({
                    let mut vec = focus_chain.clone();
                    vec.push(val);
                    (Rc::new(vec), sub_val, hl_list)
                });
            }
        })
    };

    let on_mouse_leave = {
        let on_focus_change = props.on_focus_change.clone();
        let focus_chain = Rc::new(props.focus_chain.clone());
        Callback::from(move |_e: MouseEvent| {
            on_focus_change.emit((focus_chain.clone(), None, None));
        })
    };

    let on_click = {
        let on_focus_click = props.on_focus_click.clone();
        Callback::from(move |_: MouseEvent| {
            on_focus_click.emit(());
        })
    };

    let on_child_focus = props.on_focus_change.clone();

    html! {
        <ul onmouseleave={on_mouse_leave.clone()}>
            { props.history_state.iter()
                .enumerate()
                .group_by(|(idx, (_, s))| solve_result_discriminant(*idx, s))
                .into_iter()
                .map(|(_, iter)| {
                    let content: Vec<_> = iter.into_iter().collect();
                    let ((index, (g, s)), rest) = content.split_first().unwrap();

                    let focused = props.focus_state.0.starts_with(&props.focus_chain)
                                           && props.focus_state.0.get(props.focus_chain.len()) == Some(index)
                                           && props.focus_state.1.is_none();
                    let li_class = "border dark:border-blue-400 mt-2 mr-2 dark:text-white rounded";
                    let extra_class = match (focused, props.nested) {
                        (false, false) => "font-medium bg-light-900 dark:bg-blue-100",
                        (true, false) => "font-bold dark:bg-blue-400",
                        (false, true) => "font-medium bg-light-900 dark:bg-blue-500",
                        (true, true) => "font-bold dark:bg-blue-600",
                    };
                    html! {
                        <li class={classes!(li_class, extra_class)}>
                            <div class={classes!(border_for_solution(s), "p-2", "rounded", "cursor-pointer")}
                                  onmouseover={on_mouse.clone()}
                                  onclick={on_click.clone()}
                                  data-id={index.to_string()}
                                  data-hl-list={get_hl_list(&s)}>
                                { format!("{}", s) }
                                if let SolveResults::Chain(_, _, list, _)|SolveResults::UniqueRequirement(_, _, list, _)=s {
                                    <SolutionHistory
                                        history_state={list.clone()}
                                        focus_state={props.focus_state.clone()}
                                        focus_chain={{ let mut new = props.focus_chain.clone(); new.push(*index); new }}
                                        on_focus_change={on_child_focus.clone()}
                                        on_focus_click={props.on_focus_click.clone()}
                                        nested={true} />
                                }
                                if !rest.is_empty() {
                                    <ul class="flex max-w-fit flex-wrap">

                                        { std::iter::once(&(*index, &(g.clone(), s.clone()))).chain(rest.iter())
                                            .enumerate().map(|(num, (sub_index, _))| {
                                            let focused = props.focus_state.0.starts_with(&props.focus_chain)
                                                                   && props.focus_state.0.get(props.focus_chain.len()) == Some(sub_index)
                                                                   && props.focus_state.1 == Some(num);
                                            let extra_class = if focused {
                                                "font-bold dark:bg-blue-400"
                                            } else {
                                                "font-medium bg-light-900 dark:bg-blue-100"
                                            };
                                            let num_class = "w-10 h-10 inline-block flex items-center justify-center";
                                            html!{
                                                <li class={classes!(li_class, extra_class, num_class)}>
                                                    <div class="p-2 rounded cursor-pointer w-auto"
                                                          onmouseover={on_mouse.clone()}
                                                          onclick={on_click.clone()}
                                                          data-id={sub_index.to_string()}
                                                          data-sub-id={num.to_string()}>
                                                        { format!("{}", num+1) }
                                                    </div>
                                                </li>
                                            }
                                            }).collect::<Html>()
                                        }
                                        </ul>
                                }
                            </div>
                        </li>
                    }
            }).collect::<Html>()
            }
        </ul>
    }
}
