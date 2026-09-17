Cpp Project Template With Conan + CMake + Conda

## Overview

As titled. This template can help you convieniently start a new C++ project using Conan, CMake, 
and Conda for:

* Rust libraries organization and isolation.
* Dependency management via Cargo.
* Easy integration with Python bindings.
* Quick unit-test development.

## Build Example

### Initialize Conda Environment

```shell
conda/micromamba env create -f environment.yaml -p ./_venv --yes
conda/micromamba env update -f environment.yaml -p ./_venv --yes
conda/micromamba activate ./_venv
```

### Building Example Lib and Bin

```shell
cargo build
```

## Run Examples

Run bin:

```shell
./build/build/Release/example
```

Run test:

```shell
./build/build/Release/test/test
```

Try example libraries's Python binding:
```shell
cp ./target/debug/libpy_example_lib.so ./

python -c "
import numpy as np
import py_example_lib

a = py_example_lib.add(1, 2)
b = py_example_lib.Bot()
c = b.move_to('up')
d = py_example_lib.scale(np.array([[1.0, 2.0], [3.0, 4.0]]), 2.0)
e = py_example_lib.row_means(d)

print(a)
print(b)
print(c)
print(d)
print(e)
"
```

