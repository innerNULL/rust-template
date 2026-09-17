# -*- coding: utf-8 -*-
# file: test_py_example_lib.py
# date: 2026-09-17
#
# Tests for the Python surface of `example_lib`. They cover the binding only:
# the behaviour itself is tested in Rust, under `crates/example_lib/tests`.
#
# Run them after the module is importable, e.g.
#   maturin develop --release -m bindings/python/example_lib/pyproject.toml
#   pytest bindings/python/example_lib/tests

import numpy as np
import pytest

import py_example_lib


def test_add():
    assert py_example_lib.add(1, 2) == 3


def test_bot_move_to():
    bot = py_example_lib.Bot("scout")
    assert bot.name == "scout"
    status = bot.move_to("up")
    assert isinstance(status, py_example_lib.BotStatus)
    assert status.live is True
    assert repr(status) == "BotStatus(live=True)"


def test_scale_returns_a_numpy_array():
    samples = np.array([[1.0, 3.0], [10.0, 20.0]])
    scaled = py_example_lib.scale(samples, 2.0)
    assert isinstance(scaled, np.ndarray)
    assert scaled.dtype == np.float64
    np.testing.assert_allclose(scaled, samples * 2.0)
    # The input is borrowed, not mutated.
    np.testing.assert_allclose(samples, [[1.0, 3.0], [10.0, 20.0]])


def test_row_means():
    samples = np.array([[1.0, 3.0], [10.0, 20.0]])
    np.testing.assert_allclose(py_example_lib.row_means(samples), [2.0, 15.0])


def test_row_means_rejects_an_array_without_columns():
    with pytest.raises(ValueError):
        py_example_lib.row_means(np.zeros((2, 0)))


def test_non_contiguous_input_is_accepted():
    # A transposed view is not C-contiguous; the binding must still handle it.
    samples = np.array([[1.0, 3.0], [10.0, 20.0]]).T
    np.testing.assert_allclose(py_example_lib.scale(samples, 1.0), samples)


def test_dummy_compile_option_is_off_by_default():
    assert py_example_lib.dummy_compile_option_enabled() is False
