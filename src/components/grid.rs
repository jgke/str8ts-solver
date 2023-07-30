use crate::components::cell::RenderCell;
use crate::components::requirements::RenderRequirementsCell;
use crate::diffgrid::DiffGrid;
use solver::grid::{Cell, Grid};
use yew::{function_component, html, Callback, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct RenderGridProps {
    pub grid: Grid,
    pub next_grid: Option<Grid>,
    pub show_extras: bool,
    pub on_change: Callback<((usize, usize), Cell)>,
    pub edit_mode: bool,
}

#[function_component(RenderGrid)]
pub fn render_grid(props: &RenderGridProps) -> Html {
    let diffgrid = DiffGrid::new(props.grid.clone(), props.next_grid.clone());

    html! {
        <div class="flex flex-col text-black md:p-4 mb-4 md:bg-light-800 md:dark:bg-blue-200 md:border w-fit rounded leading-tight">
            if props.show_extras {
                <div class="flex w-fit">
                    <div class="w-1/12vw h-1/12vw md:w-14 md:h-14" />
                    <div class="w-1/12vw h-1/12vw md:w-14 md:h-14" />
                    <div class="mr-2" />
                    { diffgrid.col_requirements.iter().cloned()
                        .map(|reqs| html! { <RenderRequirementsCell requirements={reqs} forbidden={false} /> }).collect::<Html>() }
                </div>
                <div class="flex w-fit mb-2">
                    <div class="w-1/12vw h-1/12vw md:w-14 md:h-14" />
                    <div class="w-1/12vw h-1/12vw md:w-14 md:h-14" />
                    <div class="mr-2" />
                    { diffgrid.col_forbidden.iter().cloned()
                        .map(|reqs| html! { <RenderRequirementsCell requirements={reqs} forbidden={true} /> }).collect::<Html>() }
                </div>
            }
            { diffgrid.cells.iter().enumerate()
                .map(|(y, row)|
                     html! {
                         <div class="flex w-fit">
                             if props.show_extras {
                                     <RenderRequirementsCell requirements={diffgrid.row_requirements[y]} forbidden={false} />
                                     <RenderRequirementsCell requirements={diffgrid.row_forbidden[y]} forbidden={true} />
                                    <div class="mr-2" />
                             }
                             {

                                 row.iter().enumerate().map(|(x, (cell, next_cell))| {
                                    let parent_on_change = props.on_change.clone();
                                    let on_change = Callback::from(move |cell| {
                                        parent_on_change.emit(((x, y), cell));
                                    });

                                    html! {
                                        <RenderCell cell={cell.clone()} next_cell={next_cell.clone()} {on_change} edit_mode={props.edit_mode} sidebar_shown={props.show_extras} />
                                    }
                                 }).collect::<Html>()
                             }
                         </div>
                     }
                ).collect::<Html>() }
        </div>
    }
}
