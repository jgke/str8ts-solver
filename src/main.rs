#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
fn main() {
    web_front::web_front()
}

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
fn main() -> std::process::ExitCode {
    str8ts_cli::cli()
}
