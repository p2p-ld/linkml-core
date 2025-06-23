# Python API Installation Guide

This repository provides Python bindings for the `linkml-core` Rust library. The bindings are built using [maturin](https://github.com/PyO3/maturin). To compile and install the package you need a working Rust toolchain and Python 3.8 or newer.

1. Install Rust from <https://www.rust-lang.org/tools/install> if it is not already available on your system.
2. Clone this repository and change into the project directory:

```bash
git clone https://github.com/linkml/rust-linkml-core.git
cd rust-linkml-core
```

3. Build and install the Python runtime module using pip:

```bash
pip install src/runtime
```

For editable installs during development use:

```bash
pip install -e src/runtime
```

After installation you should be able to import `linkml_runtime` from Python:

```python
import linkml_runtime
```
