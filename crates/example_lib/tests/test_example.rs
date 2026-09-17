//! file: test_example.rs
//! date: 2026-09-17
//!
//! Integration tests for `example_lib`, the counterpart of the C++ template's
//! `test/example_lib/test_example.h`.
//!
//! Every file directly under `tests/` is compiled into its own test binary that
//! links against the crate exactly the way a downstream user would, so only the
//! public API is reachable from here. Add one file per library module to keep
//! the mapping from `src/<module>.rs` to `tests/test_<module>.rs` obvious.

use example_lib::example::{Bot, BotStatus, add, row_means, scale};
use ndarray::array;

#[test]
fn add_matches_the_reference_case() {
    assert_eq!(add(1, 2), 3);
}

#[test]
fn bot_move_reports_a_live_status() {
    let bot = Bot::new("scout");
    assert_eq!(bot.move_to("up"), BotStatus { live: true });
}

#[test]
fn bot_status_round_trips_through_json() {
    let status = Bot::new("scout").move_to("up");
    let encoded = serde_json::to_string(&status).expect("BotStatus is serialisable");
    assert_eq!(encoded, r#"{"live":true}"#);
    let decoded: BotStatus = serde_json::from_str(&encoded).expect("BotStatus is deserialisable");
    assert_eq!(decoded, status);
}

#[test]
fn scale_and_row_means_compose() {
    let input = array![[1.0, 3.0], [10.0, 20.0]];
    let scaled = scale(input.view(), 2.0);
    assert_eq!(scaled, array![[2.0, 6.0], [20.0, 40.0]]);
    assert_eq!(row_means(scaled.view()), Some(array![4.0, 30.0]));
}

#[test]
fn dummy_compile_option_matches_the_enabled_feature() {
    assert_eq!(
        example_lib::dummy_compile_option_enabled(),
        cfg!(feature = "dummy_compile_option")
    );
}
