use solver::grid::Grid;
use wasm_bindgen::JsCast;
use web_sys::{FormData, HtmlFormElement, SubmitEvent};
use yew::{function_component, html, Callback, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct ImporterProps {
    pub on_import: Callback<Result<Grid, String>>,
}

#[function_component(Importer)]
pub fn header(props: &ImporterProps) -> Html {
    let ImporterProps { on_import } = props;

    let onsubmit = {
        let on_import = on_import.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            e.stop_propagation();
            let data = FormData::new_with_form(
                &e.target().unwrap().dyn_into::<HtmlFormElement>().unwrap(),
            )
            .unwrap();
            let field = data.get("data").as_string().unwrap();
            let grid = Grid::parse(vec![field]);
            on_import.emit(grid);
        })
    };

    html! {
        <form class="flex border w-full mb-2 flex-col bg-light-800 dark:bg-blue-300 dark:border-blue-400 rounded p-2 max-w-[538px]" {onsubmit}>
            <label class="flex flex-col">
                <span class="block dark:text-white">{"Enter the puzzle. You can either paste a link to the 'default' solver or enter a grid of numbers 1-9, letters a-i, # and . to denote solutions, blockers, walls and holes."}</span>
                <textarea class="mt-4 font-mono tracking-[1em] max-w-[90vw] text-black" rows={10} name="data" />
            </label>
            <button class="p-2 mt-4 flex-grow-0 border font-bold bg-light-800 text-black dark:border-blue-400 dark:bg-blue-300 dark:text-white rounded disabled:border-transparent disabled:text-light-300 dark:disabled:text-light-300" type="submit">{"Parse"}</button>
        </form>
    }
}
