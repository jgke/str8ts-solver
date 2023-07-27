use itertools::Itertools;

use solver::bitset::BitSet;
use yew::{classes, function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct RenderRequirementContentProps {
    pub options: BitSet,
    pub next_options: BitSet,
}

#[function_component(IntermediateCellContent)]
pub fn render_requirements_content(props: &RenderRequirementContentProps) -> Html {
    (1..=9)
        .chunks(3)
        .into_iter()
        .map(|row| {
            html! {
                <div class="w-full flex justify-evenly"> {
                    row.map(|cell| if props.options.contains(cell) || props.next_options.contains(cell) {
                        html! {
                            <span class={classes!(
                                "w-3", "text-center",
                                Some("bg-blue-300 text-white").filter(|_|!props.options.contains(cell) || !props.next_options.contains(cell))
                             )}>{format!("{}", cell)}</span>
                        }
                    } else {
                        html! { <span class="w-3 text-center"></span> }
                    }).collect::<Html>()
                }</div>
            }
        })
        .collect::<Html>()
}
