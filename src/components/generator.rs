use solver::generator;
use solver::grid::Grid;
use wasm_bindgen::JsCast;
use web_sys::{FormData, HtmlFormElement, SubmitEvent};
use yew::{function_component, html, Callback, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct GeneratorProps {
    pub on_generate: Callback<Grid>,
}

#[function_component(Generator)]
pub fn header(props: &GeneratorProps) -> Html {
    let GeneratorProps { on_generate } = props;

    let onsubmit = {
        let on_generate = on_generate.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            e.stop_propagation();
            let data = FormData::new_with_form(
                &e.target().unwrap().dyn_into::<HtmlFormElement>().unwrap(),
            )
            .unwrap();

            let size = data.get("size").as_string().unwrap().parse().unwrap_or(9);
            let blocker_count = data
                .get("blocker_count")
                .as_string()
                .unwrap()
                .parse()
                .unwrap_or(10);
            let blocker_num_count = data
                .get("blocker_num_count")
                .as_string()
                .unwrap()
                .parse()
                .unwrap_or(5);
            let target_difficulty = data
                .get("difficulty")
                .as_string()
                .unwrap()
                .parse()
                .unwrap_or(4);
            let symmetric = data.get("symmetric").as_string() == Some("on".to_string());
            let grid = generator::generator(
                size,
                blocker_count + blocker_num_count,
                blocker_num_count,
                target_difficulty,
                symmetric,
            );
            on_generate.emit(grid);
        })
    };

    html! {
        <form class="flex border w-full mb-2 flex-col bg-light-800 dark:bg-blue-300 dark:border-blue-400 rounded p-2 max-w-[538px]" {onsubmit}>
            <label class="flex flex-col">
                <span class="block dark:text-white">{"Puzzle size"}</span>
                <input class="mt-4 text-black" name="size" />
            </label>
            <label class="flex flex-col">
                <span class="block dark:text-white">{"Empty black cell count"}</span>
                <input class="mt-4 text-black" name="blocker_count" />
            </label>
            <label class="flex flex-col">
                <span class="block dark:text-white">{"Black number count"}</span>
                <input class="mt-4 text-black" name="blocker_num_count" />
            </label>
            <label class="flex flex-col">
                <span class="block dark:text-white">{"Target difficulty"}</span>
                <select class="mt-4 text-black" name="difficulty">
                    <option value="1">{"Trivial"}</option>
                    <option value="2">{"Easy"}</option>
                    <option value="3">{"Medium"}</option>
                    <option value="4">{"Hard"}</option>
                    <option value="5">{"Settis, small fishes"}</option>
                    <option value="6">{"Large fishes, short chains"}</option>
                    <option value="7">{"Long chains"}</option>
                </select>
            </label>
            <label class="flex flex-col">
                <span class="block dark:text-white">{"Symmetric"}</span>
                <input type="checkbox" class="mt-4 text-black" name="symmetric" />
            </label>
            <button class="p-2 mt-4 flex-grow-0 border font-bold bg-light-800 text-black dark:border-blue-400 dark:bg-blue-300 dark:text-white rounded disabled:border-transparent disabled:text-light-300 dark:disabled:text-light-300" type="submit">{"Parse"}</button>
        </form>
    }
}
