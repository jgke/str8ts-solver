use criterion::{black_box, criterion_group, criterion_main, Criterion};
use solver::grid::Grid;
use solver::solver::{solve_round, SolveResults, SolveType};

fn full_solve(mut grid: Grid, enable_chains: bool) -> usize {
    let orig_grid = grid.clone();
    let mut loop_count = 0;
    loop {
        loop_count += 1;
        match solve_round(&mut grid, enable_chains) {
            Ok(SolveResults {
                ty: SolveType::PuzzleSolved,
                meta: _,
            }) => {
                break;
            }
            Ok(_) => {}
            Err(e) => {
                panic!(
                    "Failed to solve grid: {}\n{}\nOriginal grid:\n{}",
                    e, grid, orig_grid
                );
            }
        }
    }
    loop_count
}

fn solver_benchmark(c: &mut Criterion) {
    let typical_grid = Grid::parse(vec![
        "12#.7...6".to_string(),
        ".#.4..3..".to_string(),
        ".#.#i....".to_string(),
        "7..3.....".to_string(),
        "9#3#.#8#7".to_string(),
        ".....1..4".to_string(),
        "....h#.#.".to_string(),
        "..9..8.#.".to_string(),
        "5...4.#32".to_string(),
    ])
    .unwrap();

    c.bench_function("grid typical puzzle", |b| {
        b.iter(|| full_solve(black_box(typical_grid.clone()), true))
    });
}

fn slow_solver_benchmark(c: &mut Criterion) {
    let chain_grid = Grid::parse(vec![
        ".#6#23...".to_string(),
        ".........".to_string(),
        "##...4...".to_string(),
        ".#8a..3..".to_string(),
        "..#...#..".to_string(),
        "..3..f9#.".to_string(),
        "...8...##".to_string(),
        ".........".to_string(),
        "...74#8#.".to_string(),
    ])
    .unwrap();

    c.bench_function("grid solve chains", |b| {
        b.iter(|| full_solve(black_box(chain_grid.clone()), true))
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = solver_benchmark
}

criterion_group! {
    name = slow_benches;
    config = Criterion::default().sample_size(10);
    targets = slow_solver_benchmark
}
criterion_main!(benches, slow_benches);
