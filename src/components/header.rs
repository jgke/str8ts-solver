use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement, MouseEvent};
use yew::{classes, function_component, html, use_state, Callback, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    pub is_solved: bool,
    pub on_step: Callback<MouseEvent>,
    pub on_hint: Callback<MouseEvent>,
    pub on_partial_solve: Callback<MouseEvent>,
    pub on_solve: Callback<MouseEvent>,

    pub set_edit_mode: Callback<bool>,
    pub edit_mode: bool,
    pub open_importer: Callback<()>,
}

#[function_component(Header)]
pub fn header(props: &HeaderProps) -> Html {
    let HeaderProps {
        is_solved,
        on_step,
        on_hint,
        on_partial_solve,
        on_solve,
        set_edit_mode,
        edit_mode,
        open_importer,
    } = props;

    let set_edit_mode = {
        let set_edit_mode = set_edit_mode.clone();
        Callback::from(move |e: Event| {
            let res = (|| {
                let val = e.target()?.dyn_into::<HtmlInputElement>().ok()?;
                Some(val.id() == "enable-edit")
            })();
            set_edit_mode.emit(res.unwrap_or(false));
        })
    };

    let hamburger_open = use_state(move || false);

    let open_hamburger = {
        let hamburger_open = hamburger_open.clone();
        Callback::from(move |_| {
            hamburger_open.set(true);
        })
    };

    let close_hamburger = {
        let hamburger_open = hamburger_open.clone();
        Callback::from(move |_| {
            hamburger_open.set(false);
        })
    };

    let on_importer_click = {
        let open_importer = open_importer.clone();
        let hamburger_open = hamburger_open.clone();
        Callback::from(move |_: MouseEvent| {
            open_importer.emit(());
            hamburger_open.set(false);
        })
    };

    let on_partial_solve_click = {
        let on_partial_solve = on_partial_solve.clone();
        let hamburger_open = hamburger_open.clone();
        Callback::from(move |e: MouseEvent| {
            on_partial_solve.emit(e);
            hamburger_open.set(false);
        })
    };

    let on_solve_click = {
        let on_solve = on_solve.clone();
        let hamburger_open = hamburger_open.clone();
        Callback::from(move |e: MouseEvent| {
            on_solve.emit(e);
            hamburger_open.set(false);
        })
    };

    let button_classes =
        "flex-grow-0 border font-bold mr-2 bg-light-800 text-black dark:border-blue-400 dark:bg-blue-300 dark:text-white rounded disabled:border-transparent disabled:text-light-300 dark:disabled:text-light-300";

    let label_classes =
        "my-2 mr-2 rounded inline-block peer-checked:font-bold w-32 flex justify-center cursor-pointer";
    let span_classes =
        "block parent-sibling-checked:border-y-4 parent-sibling-checked:border-y-blue-800 dark:parent-sibling-checked:border-y-blue-700 parent-sibling-checked:py-1 py-2 px-1 w-full text-center";

    html! {
        <div class="flex flex-wrap w-full mb-2 items-center">
            <div class="inline-block">
                <input id="disable-edit" class="hidden peer" type="radio" onchange={set_edit_mode.clone()} checked={!edit_mode} />
                <label for="disable-edit" class={classes!(button_classes, label_classes)}>
                    <span class={span_classes}>{"Edit numbers"}</span>
                </label>
            </div>
            <div class="inline-block">
                <input id="enable-edit" class="hidden peer" type="radio" onchange={set_edit_mode.clone()} checked={*edit_mode} />
                <label for="enable-edit" class={classes!(button_classes, label_classes)}>
                    <span class={span_classes}>{"Toggle color"}</span>
                </label>
            </div>
            <div class="flex-grow" />
            <button
                disabled={*is_solved}
                class={classes!(button_classes, "p-2")}
                onclick={on_step.clone()}
            >
                {"Solve 1 step"}
            </button>
            <button
                disabled={*is_solved}
                class={classes!(button_classes, "p-2")}
                onclick={on_hint.clone()}
            >
                {"Show hint"}
            </button>
            <input id="hamburger" class="hidden peer/hamburger" type="checkbox" onchange={open_hamburger} checked={*hamburger_open} />
            <label for="hamburger" class={classes!(button_classes, "cursor-pointer", "p-2", "w-[42px]", "h-[42px]")}>
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor" className="w-6 h-6">
                  <path strokeLinecap="round" strokeLinejoin="round" d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5" />
                </svg>
            </label>
            if *hamburger_open {
                <div class="fixed top-0 left-0 right-0 bottom-0 cursor-pointer" onclick={close_hamburger}></div>
            }
            if *hamburger_open {
                <div>
                    <div class="absolute flex flex-col border bg-light-700 dark:border-blue-300 dark:bg-blue-200 p-4 rounded">
                        <button
                            class={classes!(button_classes, "p-2", "mb-2")}
                            onclick={on_importer_click}
                        >
                            {"Import puzzle"}
                        </button>
                        <button
                            disabled={*is_solved}
                            class={classes!(button_classes, "p-2", "mb-2")}
                            onclick={on_partial_solve_click}
                        >
                            {"Solve without chains"}
                        </button>
                        <button
                            disabled={*is_solved}
                            class={classes!(button_classes, "p-2")}
                            onclick={on_solve_click}
                        >
                            {"Solve"}
                        </button>
                    </div>
                </div>
            }
        </div>
    }
}
