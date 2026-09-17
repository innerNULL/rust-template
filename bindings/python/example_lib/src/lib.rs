//! file: lib.rs
//! date: 2026-09-17
//!
//! PyO3 module definition, the counterpart of the C++ template's
//! `bindings/python/example_lib/src/pymodule.cpp`.
//!
//! The rule this file follows is the same one the C++ template follows: a
//! binding contains no logic. Every function here converts arguments, calls
//! `example_lib`, and converts the result back. Anything worth testing lives in
//! the library, where it can be tested without an interpreter.

// PyO3 dictates these two signatures: a `#[pymethods]` method must take `&self`
// even when the receiver is a single byte, and an argument PyO3 extracts from a
// Python object must be taken by value. Both pedantic lints would ask for code
// that does not compile, so they are switched off for this crate only.
#![allow(clippy::trivially_copy_pass_by_ref, clippy::needless_pass_by_value)]

use example_lib::example::{self, Bot, BotStatus};
use numpy::{IntoPyArray, PyArray1, PyArray2, PyReadonlyArray2};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

/// `example_lib::example::BotStatus` exposed to Python.
///
/// Foreign types cannot be `#[pyclass]`-annotated directly, so each exported
/// type gets a thin newtype wrapper here. That is also what keeps the Python
/// surface a deliberate choice instead of a mirror of every internal field.
// `frozen`: the wrapper is immutable, which lets PyO3 skip the runtime borrow
// check. `from_py_object`: opt in to extracting the class back out of a Python
// object, so it can be used as an argument type and not just as a return type.
#[pyclass(name = "BotStatus", frozen, from_py_object)]
#[derive(Debug, Clone, Copy)]
struct PyBotStatus(BotStatus);

#[pymethods]
impl PyBotStatus {
    #[new]
    #[pyo3(signature = (live = true))]
    fn new(live: bool) -> Self {
        Self(BotStatus { live })
    }

    #[getter]
    fn live(&self) -> bool {
        self.0.live
    }

    fn __repr__(&self) -> String {
        format!(
            "BotStatus(live={})",
            if self.0.live { "True" } else { "False" }
        )
    }
}

/// `example_lib::example::Bot` exposed to Python.
#[pyclass(name = "Bot", from_py_object)]
#[derive(Debug, Clone, Default)]
struct PyBot(Bot);

#[pymethods]
impl PyBot {
    #[new]
    #[pyo3(signature = (name = String::new()))]
    fn new(name: String) -> Self {
        Self(Bot::new(name))
    }

    #[getter]
    fn name(&self) -> &str {
        &self.0.name
    }

    #[setter]
    fn set_name(&mut self, name: String) {
        self.0.name = name;
    }

    fn move_to(&self, direction: &str) -> PyBotStatus {
        PyBotStatus(self.0.move_to(direction))
    }

    fn __repr__(&self) -> String {
        format!("Bot(name={:?})", self.0.name)
    }
}

/// Adds two numbers.
#[pyfunction]
fn add(a: i32, b: i32) -> i32 {
    example::add(a, b)
}

/// Multiplies every element of a 2-D `float64` array by `factor`.
///
/// `PyReadonlyArray2` borrows the NumPy buffer instead of copying it, and holds
/// a read lock on the array for as long as it is alive. The result is moved into
/// a fresh NumPy array with `into_pyarray`, which hands the allocation over
/// without copying it either.
#[pyfunction]
fn scale<'py>(
    py: Python<'py>,
    input: PyReadonlyArray2<'py, f64>,
    factor: f64,
) -> Bound<'py, PyArray2<f64>> {
    example::scale(input.as_array(), factor).into_pyarray(py)
}

/// Mean of every row of a 2-D `float64` array.
///
/// Shows the other half of the contract: a Rust `Option::None` has to become a
/// Python exception, otherwise the caller silently receives `None` and finds out
/// much later.
#[pyfunction]
fn row_means<'py>(
    py: Python<'py>,
    input: PyReadonlyArray2<'py, f64>,
) -> PyResult<Bound<'py, PyArray1<f64>>> {
    example::row_means(input.as_array())
        .map(|means| means.into_pyarray(py))
        .ok_or_else(|| PyValueError::new_err("cannot average rows of an array with no columns"))
}

/// Reports whether the extension was built with the `dummy_compile_option`
/// feature, the counterpart of the C++ template's `DUMMY_COMPILE_OPTION`.
#[pyfunction]
fn dummy_compile_option_enabled() -> bool {
    example_lib::dummy_compile_option_enabled()
}

/// Module entry point.
///
/// The function name is the module name CPython looks for, so it must stay equal
/// to `[lib] name` in `Cargo.toml`.
#[pymodule]
fn py_example_lib(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__doc__", "Python binding for the Rust `example_lib` crate")?;
    m.add_class::<PyBotStatus>()?;
    m.add_class::<PyBot>()?;
    m.add_function(wrap_pyfunction!(add, m)?)?;
    m.add_function(wrap_pyfunction!(scale, m)?)?;
    m.add_function(wrap_pyfunction!(row_means, m)?)?;
    m.add_function(wrap_pyfunction!(dummy_compile_option_enabled, m)?)?;
    Ok(())
}
