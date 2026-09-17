//! file: example.rs
//! date: 2026-09-17
//!
//! Example module: a struct with behaviour, a free function, and two `ndarray`
//! entry points that the Python binding under `bindings/python/example_lib`
//! re-exports as zero-copy NumPy functions.

use ndarray::{Array1, Array2, ArrayView2, Axis};
use serde::{Deserialize, Serialize};

/// Outcome of a [`Bot`] move.
///
/// `Serialize`/`Deserialize` are the `serde` replacement for the C++ template's
/// glaze / nlohmann_json usage: one derive and the type round-trips through JSON.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct BotStatus {
    pub live: bool,
}

/// A named bot that can be asked to move.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Bot {
    pub name: String,
}

impl Bot {
    /// Builds a bot from anything that converts into a `String`, so both
    /// `Bot::new("scout")` and `Bot::new(owned_string)` work.
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }

    /// Moves the bot and reports whether it survived the move.
    ///
    /// `tracing::debug!` is the `spdlog` replacement. Nothing is printed unless
    /// the binary installs a subscriber, so libraries can log freely.
    #[must_use]
    pub fn move_to(&self, direction: &str) -> BotStatus {
        tracing::debug!(bot = %self.name, direction, "moving bot");
        BotStatus { live: true }
    }
}

/// Adds two integers.
///
/// Uses `i32` rather than C++'s `int32_t`; the overflow behaviour is the reason
/// for `wrapping_add`-style care in real code, but a debug build panics on
/// overflow here, which is what you want in a template.
#[must_use]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Multiplies every element of `input` by `factor`.
///
/// Takes an [`ArrayView2`] rather than an owned array so the caller keeps
/// ownership of the buffer. When called from Python through the binding, that
/// view borrows the NumPy array's memory directly: no copy on the way in.
#[must_use]
// An `ArrayView` is a cheap (pointer, shape, strides) handle, and passing it by
// value is what ndarray's own API does; taking `&ArrayView2` would only add a
// level of indirection for the caller and for every read inside the loop.
#[allow(clippy::needless_pass_by_value)]
pub fn scale(input: ArrayView2<'_, f64>, factor: f64) -> Array2<f64> {
    &input * factor
}

/// Mean of every row of `input`, i.e. it reduces an `(n, m)` array to `n` values.
///
/// Returns `None` when there is nothing to average over (`m == 0`), which the
/// Python binding turns into a `ValueError`. Returning `Option` instead of
/// silently yielding `NaN` keeps the failure visible on both sides of the FFI.
#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn row_means(input: ArrayView2<'_, f64>) -> Option<Array1<f64>> {
    input.mean_axis(Axis(1))
}

// Unit tests live next to the code they cover and may touch private items.
// Cross-crate, public-API-only tests live in `../tests/` instead; both run under
// `cargo test`. `#[cfg(test)]` keeps this module out of release builds entirely.
#[cfg(test)]
mod tests {
    use super::{Bot, add, row_means, scale};
    use ndarray::{Array2, array};

    #[test]
    fn add_sums_its_arguments() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn move_to_keeps_the_bot_alive() {
        assert!(Bot::new("scout").move_to("up").live);
    }

    #[test]
    fn scale_multiplies_every_element() {
        let input = array![[1.0, 2.0], [3.0, 4.0]];
        assert_eq!(scale(input.view(), 2.0), array![[2.0, 4.0], [6.0, 8.0]]);
    }

    #[test]
    fn row_means_averages_each_row() {
        let input = array![[1.0, 3.0], [10.0, 20.0]];
        assert_eq!(row_means(input.view()), Some(array![2.0, 15.0]));
    }

    #[test]
    fn row_means_reports_empty_rows() {
        let empty = Array2::<f64>::zeros((2, 0));
        assert_eq!(row_means(empty.view()), None);
    }
}
