extern crate web_sys;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
#[macro_export]
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[macro_export]
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
macro_rules! log {
    ( $( $t:tt )* ) => {
        println!("{}", format!( $( $t )* ));
    }
}
