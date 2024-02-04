use crate::components::requirements_content::IntermediateCellContent;
use itertools::Itertools;

use solver::bitset::BitSet;
use solver::grid::Cell;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, FocusEvent, HtmlElement, HtmlTextAreaElement, MouseEvent};
use yew::{
    classes, function_component, html, use_effect_with, use_node_ref, use_state, Callback,
    Html, Properties,
};

#[derive(Properties, PartialEq)]
pub struct RenderCellProps {
    pub cell: Cell,
    pub next_cell: Cell,
    pub on_change: Callback<Cell>,
    pub edit_mode: bool,
    pub sidebar_shown: bool,
    pub hl: bool,
}

#[function_component(RenderCell)]
pub fn render_cell(props: &RenderCellProps) -> Html {
    let cell_size = if props.sidebar_shown {
        "w-1/12vw h-1/12vw md:w-14 md:h-14"
    } else {
        "w-1/10vw h-1/10vw md:w-14 md:h-14"
    };

    let content_base =
        "flex items-center justify-center w-full h-full font-bold text-[6vw] md:text-3xl";
    let mut next = props.next_cell.clone();
    if let (Cell::Indeterminate(_), Cell::Solution(n)) = (&props.cell, &props.next_cell) {
        next = Cell::Indeterminate([*n].into_iter().collect());
    }
    let editing = use_state(move || false);
    let initial_value = match &props.cell {
        Cell::Requirement(n) | Cell::Solution(n) | Cell::Blocker(n) => n.to_string(),
        Cell::Indeterminate(nums) => nums
            .into_iter()
            .sorted()
            .map(|c| c.to_string())
            .collect::<String>(),
        Cell::Black => "".to_string(),
    };
    let onmouseup = {
        let parent_on_change = props.on_change.clone();
        let editing = editing.clone();
        let cell = Rc::new(props.cell.clone());
        let edit_mode = props.edit_mode;
        Callback::from(move |e: MouseEvent| {
            if e.button() == 2 || edit_mode {
                let new_cell = match &*cell {
                    Cell::Requirement(n) | Cell::Solution(n) => Cell::Blocker(*n),
                    Cell::Blocker(n) => Cell::Requirement(*n),
                    Cell::Indeterminate(_) => Cell::Black,
                    Cell::Black => Cell::Indeterminate((1..=9).collect()),
                };
                parent_on_change.emit(new_cell);
            } else {
                editing.set(true);
            }
        })
    };
    let onblur = {
        let parent_on_change = props.on_change.clone();
        let editing = editing.clone();
        let cell = Rc::new(props.cell.clone());

        Callback::from(move |e: FocusEvent| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlTextAreaElement>().ok());
            let field_value = input.unwrap().value();
            let field_value = &field_value;

            editing.set(false);
            let nums = (*field_value)
                .chars()
                .filter_map(|c| c.to_digit(10))
                .map(|c| c as u8)
                .collect::<BitSet>();

            let new_cell = match (&*cell, nums.len()) {
                (Cell::Requirement(_), 1)
                | (Cell::Solution(_), 1)
                | (Cell::Indeterminate(_), 1) => {
                    Cell::Requirement(nums.into_iter().next().unwrap())
                }
                (Cell::Requirement(_), _)
                | (Cell::Solution(_), _)
                | (Cell::Indeterminate(_), _) => Cell::Indeterminate(nums),
                (Cell::Blocker(_), 1) | (Cell::Black, 1) => {
                    Cell::Blocker(nums.into_iter().next().unwrap())
                }
                (Cell::Blocker(_), _) | (Cell::Black, _) => Cell::Black,
            };
            parent_on_change.emit(new_cell);
        })
    };

    let input_ref = use_node_ref();
    {
        let input_ref = input_ref.clone();
        use_effect_with(
            (input_ref, *editing),
            |(input_ref, _)| {
                if let Some(input) = input_ref.cast::<HtmlElement>() {
                    _ = input.focus();
                }
            },
        );
    }

    let onfocus = {
        Callback::from(move |e: FocusEvent| {
            if let Some(target) = e
                .target()
                .and_then(|t| t.dyn_into::<HtmlTextAreaElement>().ok())
            {
                target.select();
            }
        })
    };

    if *editing {
        return html! {
            <div class={classes!("border", "border-primary", cell_size)}>
                <textarea class={classes!(cell_size, "resize-none", "border-none", "bg-none")} ref={input_ref} {onfocus} autofocus=true {onblur} value={initial_value} />
            </div>
        };
    }

    let oncontextmenu = Callback::from(|e: MouseEvent| e.prevent_default());
    let content = match (&props.cell, &next) {
        (Cell::Requirement(cell), _) => {
            html! { <span {onmouseup} class={classes!(content_base, "bg-white")}>{cell} </span> }
        }
        (Cell::Blocker(cell), _) => {
            html! { <span {onmouseup} class={classes!(content_base, "bg-primary-black", "text-white")}>{cell} </span> }
        }
        (Cell::Black, _) => {
            html! { <span {onmouseup} class={classes!(content_base, "bg-primary-black", "text-white")}></span> }
        }
        (Cell::Solution(cell), _) => {
            html! { <span {onmouseup} class={classes!(content_base, "bg-white", "text-blue-600", "italic")}>{cell} </span> }
        }
        (Cell::Indeterminate(options), Cell::Indeterminate(next_options)) => {
            if options.is_empty() {
                html! {
                    <span {onmouseup} class={classes!(content_base, "bg-error")}>
                        {"\u{2205}"}
                    </span>
                }
            } else if props.hl {
                html! {
                    <div {onmouseup} class="flex justify-evenly w-full h-full bg-error flex-col text-[2vw] md:text-xs p-1">
                        <IntermediateCellContent options={*options} next_options={*next_options} />
                    </div>
                }
            } else {
                html! {
                    <div {onmouseup} class="flex justify-evenly w-full h-full bg-white flex-col text-[2vw] md:text-xs p-1">
                        <IntermediateCellContent options={*options} next_options={*next_options} />
                    </div>
                }
            }
        }
        (Cell::Indeterminate(options), _) => {
            if options.is_empty() {
                html! {
                    <span {onmouseup} class={classes!(content_base, "bg-error")}>
                        {"\u{2205}"}
                    </span>
                }
            } else if props.hl {
                html! {
                    <div {onmouseup} class="flex justify-evenly w-full h-full bg-error flex-col text-[2vw] md:text-xs p-1">
                        <IntermediateCellContent options={*options} next_options={*options} />
                    </div>
                }
            } else {
                html! {
                    <div {onmouseup} class="flex justify-evenly w-full h-full bg-white flex-col text-[2vw] md:text-xs p-1">
                        <IntermediateCellContent options={*options} next_options={*options} />
                    </div>
                }
            }
        }
    };
    html! {
        <div class={classes!("border", "border-primary", "dark:border-blue-300", cell_size)} {oncontextmenu}>
            { content }
        </div>
    }
}
