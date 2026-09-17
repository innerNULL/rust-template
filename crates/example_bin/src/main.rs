//! file: main.rs
//! date: 2026-09-17
//!
//! Example program, the counterpart of the C++ template's `src/bin/example.cpp`.
//!
//! It exists to show the wiring, not the logic: a binary installs the process
//! wide services (here: logging) and then calls into the libraries.

use example_lib::example::{Bot, add, row_means, scale};
use ndarray::array;
use tracing_subscriber::EnvFilter;

fn main() {
    // The `spdlog::set_level` equivalent. Installing the subscriber is the
    // binary's job: until one exists, every `tracing` call in every library is a
    // no-op. `RUST_LOG=debug ./example` turns the library's debug lines on.
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    tracing::info!(
        dummy_compile_option = example_lib::dummy_compile_option_enabled(),
        "example starting"
    );

    println!("The answer of 1 + 1 = {}", add(1, 1));

    let bot = Bot::new("scout");
    let status = bot.move_to("up");
    // `serde_json` replaces glaze / nlohmann_json for structured output.
    match serde_json::to_string(&status) {
        Ok(encoded) => println!("{} moved up -> {encoded}", bot.name),
        Err(err) => tracing::error!(%err, "failed to serialise bot status"),
    }

    let samples = array![[1.0, 3.0], [10.0, 20.0]];
    let scaled = scale(samples.view(), 2.0);
    println!("scaled samples = {scaled}");
    match row_means(scaled.view()) {
        Some(means) => println!("row means = {means}"),
        None => println!("row means = <no columns to average>"),
    }
}
