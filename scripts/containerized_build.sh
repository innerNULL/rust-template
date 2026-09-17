# -*- coding: utf-8 -*-
# file: containerized_build.sh
# date: 2026-09-17
#
# Builds the workspace inside a container so the produced binaries and wheels
# match the target platform instead of the developer's machine. Usage:
#
#   bash ./scripts/containerized_build.sh ./scripts/containerized_build.centos.ini
#
# The .ini file is sourced, not parsed: it is plain shell, so it can compute
# paths. See containerized_build.centos.ini for the variables it must define.


set -x
set -euo pipefail


source $1


# Copies only what the build needs into a scratch workspace. Everything listed
# here is an input; `target/` deliberately is not, so a container build never
# reuses host artifacts compiled against the host's glibc.
function init {
  rm -rf ${WORKSPACE}
  mkdir -p ${WORKSPACE}
  cp Cargo.toml ${WORKSPACE}
  # Copying the lockfile is what makes the container build reproducible: the
  # container resolves nothing, it builds the exact versions the host resolved.
  cp Cargo.lock ${WORKSPACE}
  cp rust-toolchain.toml ${WORKSPACE}
  cp rustfmt.toml ${WORKSPACE}
  cp -r ./.cargo ${WORKSPACE}
  cp -r ./crates ${WORKSPACE}
  cp -r ./bindings ${WORKSPACE}
  cp ${CONDA_YAML_PATH} ${WORKSPACE}/environment.yaml
  cp ${REQUIREMENTS_PATH} ${WORKSPACE}/requirements.txt
  cp ${BUILD_IN_CONTAINER_SCRIPT_PATH} ${WORKSPACE}
}


# TODO: Add more custom compiling options around `-e CARGO_FEATURES=...` based on need
function main {
  init
  local build_script=$(basename ${BUILD_IN_CONTAINER_SCRIPT_PATH})
  podman run -it --rm \
    -v ${WORKSPACE}:/workspace \
    -w /workspace \
    -e CARGO_FEATURES="${CARGO_FEATURES}" \
    -e CARGO_PROFILE="${CARGO_PROFILE}" \
    ${IMAGE} /bin/bash -c "cd /workspace && bash ./${build_script}"
}


main
