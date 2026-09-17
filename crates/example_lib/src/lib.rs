//! file: lib.rs
//! date: 2026-09-17
//!
//! Crate root of `example_lib`.
//!
//! In C++ the equivalent file would be the public header that `src/example_lib`
//! exports. Rust has no headers: the module tree *is* the public interface, so
//! this file only declares which modules exist and which of them are public.
//!
//! The C++ template nests everything in
//! `cpp_cmake_conan_template::example_lib::example`. Here the crate name is the
//! first path segment already, so the same item is reached as
//! [`example_lib::example::add`](crate::example::add).

pub mod example;

/// Whether the crate was compiled with the `dummy_compile_option` feature.
///
/// This is the counterpart of the C++ template's `DUMMY_COMPILE_OPTION` define.
/// Prefer this over sprinkling `#[cfg(feature = ...)]` across call sites: the
/// check stays in one place and both branches keep compiling.
#[must_use]
pub const fn dummy_compile_option_enabled() -> bool {
    cfg!(feature = "dummy_compile_option")
}
