use clap::CommandFactory;
use clap::Parser;
use solver::difficulty::get_puzzle_difficulty;
use solver::generator;
use solver::grid::Grid;
use solver::solve_result::{SolveResults, SolveType};
use solver::solver::solve_round;
use solver::strategy::StrategyList;
use std::process::ExitCode;

pub fn cli() -> ExitCode {
    /// Generate or solve a srt8ts puzzle.
    #[derive(Parser, Debug)]
    #[command(author, version, about, long_about = None)]
    struct Args {
        /// Generate a puzzle
        #[arg(long, default_value_t = false)]
        generate: bool,
        /// Solve a puzzle
        #[arg(long, default_value_t = false)]
        solve: bool,
        /// generate: Size of the puzzle
        #[arg(long, default_value_t = 9)]
        size: usize,
        /// generate: Amount of black squares in the puzzle
        #[arg(long, default_value_t = 15)]
        blocker_count: usize,
        /// generate: Amount of numbers inside black squares
        #[arg(long, default_value_t = 5)]
        blocker_num_count: usize,
        /// generate: Target difficulty in stars.
        #[arg(long, default_value_t = 5)]
        target_difficulty: usize,
        /// generate: Should the puzzle be unsymmetric
        #[arg(long, default_value_t = false)]
        not_symmetric: bool,
        /// solve: Puzzle to be solved
        #[arg(long)]
        puzzle: Option<String>,
        #[arg(long, default_value_t = false)]
        silent: bool,
    }

    let Args {
        generate,
        solve,
        size,
        blocker_count,
        blocker_num_count,
        target_difficulty,
        not_symmetric,
        puzzle,
        silent,
    } = Args::parse();

    use log::info;
    env_logger::init_from_env(env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "warn"));

    if (generate && solve) || !(generate || solve) {
        println!("Error: Pass either --generate or --solve\n");
        let _ = Args::command().print_help();
        return ExitCode::FAILURE;
    }

    if generate {
        info!(
            "Generating a puzzle with difficulty {}, this may take a few moments...",
            target_difficulty
        );
        info!(
            "Settings: size={}x{}, {} black squares of which {} contain numbers",
            size, size, blocker_count, blocker_num_count
        );

        let grid = generator::generator(size, blocker_count, blocker_num_count, target_difficulty, !not_symmetric);
        info!("Generated grid with difficulty {}", target_difficulty);
        info!("Strats required: {:#?}", get_puzzle_difficulty(&grid, &StrategyList::all()).unwrap());
        println!("{}", grid);
    } else if solve {
        if let Some(puzzle) = puzzle {
            let mut grid = match Grid::parse(vec![puzzle.clone()]) {
                Ok(grid) => grid,
                Err(e) => {
                    println!("Failed to parse grid: {}", e);
                    return ExitCode::FAILURE;
                }
            };
            info!("Solving puzzle");
            info!("\n{}", grid);
            info!("Steps:");

            let mut step_count = 0;
            loop {
                match solve_round(&mut grid, true) {
                    Ok(SolveResults {
                        ty: SolveType::PuzzleSolved,
                        meta: _,
                    }) => {
                        break;
                    }
                    Ok(step) => {
                        step_count += 1;
                        info!("{}: {}", step_count, step);
                    }
                    Err(e) => {
                        println!("Failed to solve grid: {}", e);
                        println!("Original puzzle: {}", puzzle);
                        return ExitCode::FAILURE;
                    }
                }
            }

            info!("Solved grid in {} steps", step_count);
            if !silent {
                println!("{}", grid);
            }
        } else {
            println!("Error: argument puzzle required for solving");
            let _ = Args::command().print_help();
            return ExitCode::FAILURE;
        }
    }

    ExitCode::SUCCESS
}
