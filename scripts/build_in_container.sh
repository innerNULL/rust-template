# file: build_in_container.sh
# date: 2026-09-17
#
# Runs *inside* the container, in the scratch workspace that
# containerized_build.sh populated. Not meant to be run on a host directly.


set -x
set -euo pipefail

CARGO_FEATURES="${CARGO_FEATURES:-}"
CARGO_PROFILE="${CARGO_PROFILE:-release}"


# Install micromamba
curl -L micro.mamba.pm/install.sh | bash -s -- -y
source ~/.bashrc

# Initialize conda environment. This brings its own rustc/cargo, so the container
# image does not need a Rust toolchain of its own.
micromamba env create -f environment.yaml -p ./_venv --yes
micromamba activate /workspace/_venv

# Install the Python side (maturin, numpy, pytest).
pip install -r requirements.txt

# Build the workspace.
# `--locked` fails instead of silently updating Cargo.lock: a container build
# must produce the same dependency graph as the host build it mirrors.
# TODO: Add more compiling options around `--features ...` based on need
if [ -n "${CARGO_FEATURES}" ]; then
  cargo build --locked --workspace --profile "${CARGO_PROFILE}" --features "${CARGO_FEATURES}"
else
  cargo build --locked --workspace --profile "${CARGO_PROFILE}"
fi

# Build the Python wheel for this platform. It lands in ./target/wheels.
maturin build --locked --release -m bindings/python/example_lib/Cargo.toml
