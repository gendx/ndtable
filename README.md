# ndtable: simple dense and sparse multi-dimensional tables

[![Crate](https://img.shields.io/crates/v/ndtable.svg?logo=rust)](https://crates.io/crates/ndtable)
[![Documentation](https://img.shields.io/docsrs/ndtable/0.1.2?logo=rust)](https://docs.rs/ndtable/0.1.2/)
[![Minimum Rust 1.91.0](https://img.shields.io/crates/msrv/ndtable/0.1.2.svg?logo=rust&color=orange)](https://releases.rs/docs/1.91.0/)
[![Lines of Code](https://www.aschey.tech/tokei/github/gendx/ndtable?category=code&branch=0.1.2)](https://github.com/gendx/ndtable/tree/0.1.2)
[![Dependencies](https://deps.rs/crate/ndtable/0.1.2/status.svg)](https://deps.rs/crate/ndtable/0.1.2)
[![License](https://img.shields.io/crates/l/ndtable/0.1.2.svg)](https://github.com/gendx/ndtable/blob/0.1.2/LICENSE)
[![Codecov](https://codecov.io/gh/gendx/ndtable/branch/0.1.2/graph/badge.svg)](https://codecov.io/gh/gendx/ndtable/tree/0.1.2)
[![Build Status](https://github.com/gendx/ndtable/actions/workflows/build.yml/badge.svg?branch=0.1.2)](https://github.com/gendx/ndtable/actions/workflows/build.yml)
[![Test Status](https://github.com/gendx/ndtable/actions/workflows/tests.yml/badge.svg?branch=0.1.2)](https://github.com/gendx/ndtable/actions/workflows/tests.yml)

This crate implements simple multi-dimensional tables, for use in dynamic
programming algorithms, memoization, etc. It offers two implementations: a dense
table `TableNd` and a sparse table `SparseTableNd`.
