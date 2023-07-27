use crate::generator::get_puzzle_difficulty;
use rand::{thread_rng, Rng};

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
    let final_grid;
    loop {
        iterations += 1;
        let size = 9;
        let blocker_count = thread_rng().gen_range(12..=15);
        let blocker_num_count = thread_rng().gen_range(3..=5);
        let target_difficulty = 5;
        if iterations % 10000 == 0 {
            print!(".");
            let _ = std::io::stdout().flush();
        }
        match generator::generator(
            size,
            blocker_count,
            blocker_num_count,
            target_difficulty,
            true,
        ) {
            None => {}
            Some((grid, difficulty)) => {
                println!("\nGenerated grid with difficulty {}:\n{}", difficulty, grid);
                if difficulty == target_difficulty {
                    final_grid = grid;
                    break;
                }
            }
        }
    }
    println!(
        "Strats: {:#?}",
        get_puzzle_difficulty(&final_grid, true).unwrap()
    );
}
