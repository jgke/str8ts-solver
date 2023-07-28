#[macro_use]
mod utils;

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
mod components;
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
mod diffgrid;

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
fn main() {
    let root = gloo::utils::document().get_element_by_id("wasm-render-target");
    if let Some(root) = root {
        yew::Renderer::<components::app::App>::with_root(root).render();
    } else {
        yew::Renderer::<components::app::App>::new().render();
    }
}

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
fn main() {
    use solver::difficulty::get_puzzle_difficulty;
    use solver::generator;

    let size = 9;
    let blocker_count = 15; //thread_rng().gen_range(12..=15);
    let blocker_num_count = 5; //thread_rng().gen_range(3..=5);
    let target_difficulty = 5;

    let grid = generator::generator(
        size,
        blocker_count,
        blocker_num_count,
        target_difficulty,
        true,
    );
    println!(
        "\nGenerated grid with difficulty {}:\n{}",
        target_difficulty, grid
    );
    println!("Strats: {:#?}", get_puzzle_difficulty(&grid, true).unwrap());
}
