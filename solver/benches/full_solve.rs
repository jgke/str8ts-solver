use criterion::{black_box, criterion_group, criterion_main, Criterion};
use solver::grid::Grid;
use solver::solver::{solve_round, SolveResults};

fn full_solve(mut grid: Grid) -> usize {
    let mut loop_count = 0;
    loop {
        loop_count += 1;
        match solve_round(&mut grid) {
            Ok(SolveResults::PuzzleSolved) => {
                break;
            }
            Ok(_) => {}
            Err(e) => {
                panic!("{}", e);
            }
        }
    }
    loop_count
}

fn criterion_benchmark(c: &mut Criterion) {
    let grid = Grid::parse(vec![
        "#12.4.7..".to_string(),
        "#.......5".to_string(),
        "#.i#e....".to_string(),
        "..#.#.6..".to_string(),
        "#9..#..3#".to_string(),
        "..6.#.#..".to_string(),
        "....a#c.#".to_string(),
        "2.......#".to_string(),
        "..5.6.21#".to_string(),
    ])
    .unwrap();

    c.bench_function("grid solve chains", |b| {
        b.iter(|| full_solve(black_box(grid.clone())))
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = criterion_benchmark
}
criterion_main!(benches);
