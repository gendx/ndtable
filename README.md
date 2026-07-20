# ndtable: simple dense and sparse multi-dimensional tables

[![Minimum Rust 1.91.0](https://img.shields.io/badge/rust-1.91.0%2B-orange.svg?logo=rust)](https://releases.rs/docs/1.91.0/)
[![Lines of Code](https://www.aschey.tech/tokei/github/gendx/ndtable?category=code&branch=main)](https://github.com/gendx/ndtable)
[![Dependencies](https://deps.rs/repo/github/gendx/ndtable/status.svg)](https://deps.rs/repo/github/gendx/ndtable)
[![License](https://img.shields.io/crates/l/ndtable.svg)](https://github.com/gendx/ndtable/blob/main/LICENSE)
[![Codecov](https://codecov.io/gh/gendx/ndtable/branch/main/graph/badge.svg)](https://codecov.io/gh/gendx/ndtable/tree/main)
[![Build Status](https://github.com/gendx/ndtable/actions/workflows/build.yml/badge.svg?branch=main)](https://github.com/gendx/ndtable/actions/workflows/build.yml)
[![Test Status](https://github.com/gendx/ndtable/actions/workflows/tests.yml/badge.svg?branch=main)](https://github.com/gendx/ndtable/actions/workflows/tests.yml)

This crate implements simple multi-dimensional tables, for use in dynamic
programming algorithms, memoization, etc. It offers two implementations: a dense
table `TableNd` and a sparse table `SparseTableNd`.
