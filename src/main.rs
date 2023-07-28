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
    use clap::Parser;
    use solver::difficulty::get_puzzle_difficulty;
    use solver::generator;
    /// Generate a srt8ts puzzle.
    #[derive(Parser, Debug)]
    #[command(author, version, about, long_about = None)]
    struct Args {
        /// Size of the puzzle
        #[arg(long, default_value_t = 9)]
        size: usize,
        /// Amount of black squares in the puzzle
        #[arg(long, default_value_t = 15)]
        blocker_count: usize,
        /// Amount of numbers inside black squares
        #[arg(long, default_value_t = 5)]
        blocker_num_count: usize,
        /// Target difficulty in stars.
        #[arg(long, default_value_t = 5)]
        target_difficulty: usize,
        /// Should the puzzle be unsymmetric
        #[arg(long, default_value_t = true)]
        not_symmetric: bool,
    }

    let Args {
        size,
        blocker_count,
        blocker_num_count,
        target_difficulty,
        not_symmetric,
    } = Args::parse();

    use log::info;
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "warn"),
    );

    info!(
        "Generating a puzzle with difficulty {}, this may take a few moments...",
        target_difficulty
    );
    info!(
        "Settings: size={}x{}, {} black squares of which {} contain numbers",
        size, size, blocker_count, blocker_num_count
    );

    let grid = generator::generator(
        size,
        blocker_count,
        blocker_num_count,
        target_difficulty,
        !not_symmetric,
    );
    info!("Generated grid with difficulty {}", target_difficulty);
    info!(
        "Strats required: {:#?}",
        get_puzzle_difficulty(&grid, true).unwrap()
    );
    println!("{}", grid);
}
