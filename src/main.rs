// Setup Clippy
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
#![allow(unknown_lints)]
#![warn(missing_debug_implementation)]
#![warn(missing_copy_implementation)]
#![warn(rust_2018_idioms)]
#![warn(rust_2021_compatibility)]
#![warn(trivial_casts, trivial_numeric_casts)]
#![warn(unused_qualifications)]
#![warn(variant_size_difference)]

use env_logger::{Builder, Env};
use std::thread;

use finnit_backend::App as Backend;
use finnit_frontend::App as Frontend;

fn main() {
    init_logging();

    let (mut backend, backend_rx) = Backend::new();
    let (mut frontend, frontend_rx) = Frontend::new();

    backend.listen(frontend_rx);
    frontend.listen(backend_rx);

    let svc_backend = thread::spawn(move || {
        backend.run();
    });

    let svc_frontend = thread::spawn(move || {
        frontend.run().unwrap();
    });

    svc_backend.join().unwrap();
    svc_frontend.join().unwrap();
}

fn init_logging() {
    // Log to disk
    let log_file = std::fs::File::create("finnit.log").expect("Could not create finnit.log");
    let env = Env::default()
        .filter_or("LOG_LEVEL", "info")
        .write_style_or("LOG_STYLE", "always");

    Builder::from_env(env)
        .target(env_logger::Target::Pipe(Box::new(log_file)))
        .init();
}
