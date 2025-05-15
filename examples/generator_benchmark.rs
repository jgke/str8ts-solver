use criterion::black_box;
use solver::generator::generator_loop;
use std::time::Instant;

fn bench_function<T, F: FnOnce() -> T>(msg: &str, f: F) -> T {
    let start = Instant::now();
    println!("{}...", msg);
    let res = f();
    println!("{}: {}s", msg, start.elapsed().as_secs_f64());
    black_box(res)
}

fn bench_generator(msg: &str, difficulty: usize, seed: u64) {
    bench_function(msg, || {
        generator_loop(
            black_box(9),
            black_box(15),
            black_box(5),
            black_box(difficulty),
            black_box(true),
            black_box(seed),
        )
    });
}

const SEEDS: [(usize, u64); 5] = [
    /* these take 2 tries */
    (1, 9353087330078473652),
    (2, 40474797210651839),
    (4, 12253098362932368747),
    (5, 11772944118207130575),
    /* this succeeds with the first seed */
    (6, 12523943786140325652),
];

fn main() {
    for (difficulty, seed) in SEEDS {
        bench_generator(&format!("generator diff={}", difficulty), difficulty, seed);
    }
}

#[cfg(test)]
mod tests {
    fn test_i(i: usize) {
        super::bench_generator(&format!("generator diff={}", i + 1), i + 1, super::SEEDS[i - 1].1);
    }

    #[test]
    fn test_1() {
        test_i(1);
    }
    #[test]
    fn test_2() {
        test_i(2);
    }
    #[test]
    fn test_4() {
        test_i(4);
    }
    #[test]
    fn test_5() {
        test_i(5);
    }
    #[test]
    fn test_6() {
        test_i(6);
    }
}
