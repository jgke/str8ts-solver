#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
use str8ts_solver::worker::codec::TransferrableCodec;
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
use str8ts_solver::worker::HashWorker;

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
use gloo_worker::Registrable;

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
fn main() {
    console_error_panic_hook::set_once();
    let _ = console_log::init_with_level(log::Level::Debug);
    HashWorker::registrar()
        .encoding::<TransferrableCodec>()
        .register();
}

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
fn main() {}
