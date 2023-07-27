use crate::components::requirements_content::IntermediateCellContent;

use solver::bitset::BitSet;
use yew::{classes, function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct RenderRequirementsProps {
    pub forbidden: bool,
    pub requirements: (BitSet, BitSet),
}

#[function_component(RenderRequirementsCell)]
pub fn render_requirements(props: &RenderRequirementsProps) -> Html {
    html! {
    <div class="border border-light-800 dark:border-blue-300 w-1/12vw h-1/12vw md:w-14 md:h-14">
        <div class={classes!("flex", "justify-evenly", "w-full", "h-full", if props.forbidden { "bg-primary-black dark:bg-black text-white" } else { "bg-primary-light dark:bg-primary" }, "flex-col", "dark:text-white", "text-[2vw]", "md:text-xs", "p-1")}>
            <IntermediateCellContent options={props.requirements.0} next_options={props.requirements.1} />
        </div>
    </div>
    }
}
