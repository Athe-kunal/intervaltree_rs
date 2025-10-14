# Intervaltree in rust

This crate exposes an interval tree implementation written in Rust to Python via [PyO3](https://pyo3.rs/). The Python wrapper provides the ability to build a tree from tuples, insert additional intervals, search for overlaps, and delete intervals by their `(left, right)` key.

## Requirements

- Rust toolchain (for compiling the extension module)
- Python 3.8+
- [maturin](https://github.com/PyO3/maturin) for building/installing the package

## Quick start

```bash
python -m venv .venv
source .venv/bin/activate
pip install maturin
maturin develop
```

`maturin develop` builds the extension module in-place and installs it into the active virtual environment, making it importable as `intervaltree_rs`.

## Usage

Once installed, you can use the interval tree directly from Python:

```python
from intervaltree_rs import IntervalTree

# Build a tree from tuples: (left, right, payload)
intervals = [
    (5, 10, "a"),
    (12, 18, "b"),
    (1, 4, "c"),
]
tree = IntervalTree.from_tuples(intervals)

# Insert another interval
tree.insert((8, 11, "d"))

# Search for overlaps. Inclusive bounds are enabled by default.
hits = tree.search(9, 10)
for left, right, value in hits:
    print(left, right, value)

# Delete by the interval key
removed = tree.delete((12, 18))
print("Removed:", removed)
```

### Search options

`IntervalTree.search(ql, qr, inclusive=True)` accepts an `inclusive` flag. Set it to `False` to perform exclusive range queries.

## Building a distributable wheel

To build a wheel that you can distribute or upload to PyPI, run:

```bash
maturin build --release
```

The built wheels will be placed under `target/wheels/`.

## Running tests

The Python bindings are covered by Rust unit tests. Run them with:

```bash
cargo test
```
