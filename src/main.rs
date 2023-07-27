#[macro_use]
mod utils;
mod difficulty;

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
mod generator;
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
fn main() {
    use std::io::Write;

    let mut iterations = 0;
    let size = 9;
    let blocker_count = 20;
    let blocker_num_count = 4;
    let target_difficulty = 7;
    loop {
        match generator::generator(
            size,
            blocker_count,
            blocker_num_count,
            target_difficulty,
            true,
        ) {
            None => {
                iterations += 1;
                if iterations % 10000 == 0 {
                    print!(".");
                    let _ = std::io::stdout().flush();
                }
            }
            Some((grid, difficulty)) => {
                println!("\nGenerated grid with difficulty {}:\n{}", difficulty, grid);
                if difficulty == target_difficulty {
                    break;
                }
            }
        }
    }
}
