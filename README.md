# str8ts-solver

This is a graphical str8ts solver. It can also be used to generate new puzzles from command line.

## Usage

You need Rust and Cargo installed.

### Generating puzzles

```
$ cargo run --release # this takes some time
    Finished release [optimized + debuginfo] target(s) in 0.06s
     Running `target/release/str8ts-solver`
5216.....
..#..85..
..#8a.2..
4....7.#2
...#59...
.a.2..6#.
8.g....#.
.##......
......#i.
```

See `cargo run --release -- --help` for more usage. Try `cargo run --release --
--target-difficulty 7` for harder puzzles, and `cargo run --release --
--target-difficulty 4` for easier ones. Note that the puzzles with difficulty 6
and 7 take significantly more time to generate.

Use `RUST_LOG=debug cargo run --release -- --target-difficulty 7` to see some progress.

### Graphical solver

The solver is implemented as a Yew app, and requires the `trunk` and the `wasm32` targets to be installed:

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk wasm-bindgen-cli
```

After those are installed, you can start the UI with

```bash
trunk serve --release
```

### Release

```bash
trunk build --release
```

This builds the app in release mode similar to `cargo build --release`.

The output will be located in the `dist` directory.

### Running tests

Run tests for the solver:
```
$ cargo test --all
```

Run e2e tests:

```
$ trunk serve --release
# ... in other terminal:
$ cd endtoend-tests
$ npm install
$ npx playwright test
```

### License

AGPL, see LICENSE
