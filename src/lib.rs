//! This crate implements simple multi-dimensional tables, for use in dynamic
//! programming algorithms, memoization, etc.
//!
//! It exposes two flavors:
//! - [`TableNd`] is a plain table that allocates one slot per value,
//! - [`SparseTableNd`] is a sparse table that uses a hash table under the hood
//!   to only allocate slots based on actual usage.
//!
//! Use the former if your table is dense, as indexing is faster, and the latter
//! if your table is sparse, as memory usage is lower.

#![forbid(missing_docs, unsafe_code)]

use std::collections::HashMap;
use std::hash::{BuildHasher, RandomState};
use std::ops::{Index, IndexMut};

/// A dense N-dimensional table.
pub struct TableNd<T, const N: usize> {
    size: [usize; N],
    matrix: Box<[T]>,
}

/// A dense 2-dimensional table.
pub type Table2d<T> = TableNd<T, 2>;
/// A dense 3-dimensional table.
pub type Table3d<T> = TableNd<T, 3>;
/// A dense 4-dimensional table.
pub type Table4d<T> = TableNd<T, 4>;
/// A dense 5-dimensional table.
pub type Table5d<T> = TableNd<T, 5>;
/// A dense 6-dimensional table.
pub type Table6d<T> = TableNd<T, 6>;
/// A dense 7-dimensional table.
pub type Table7d<T> = TableNd<T, 7>;
/// A dense 8-dimensional table.
pub type Table8d<T> = TableNd<T, 8>;
/// A dense 9-dimensional table.
pub type Table9d<T> = TableNd<T, 9>;
/// A dense 10-dimensional table.
pub type Table10d<T> = TableNd<T, 10>;
/// A dense 11-dimensional table.
pub type Table11d<T> = TableNd<T, 11>;
/// A dense 12-dimensional table.
pub type Table12d<T> = TableNd<T, 12>;

impl<T: Default + Clone, const N: usize> TableNd<T, N> {
    /// Creates a new dense table initialized with default values.
    pub fn new(size: [usize; N]) -> Self {
        Self {
            size,
            matrix: vec![T::default(); product(&size)].into_boxed_slice(),
        }
    }
}

impl<T: Clone, const N: usize> TableNd<T, N> {
    /// Creates a new dense table initialized with the given value.
    pub fn init(value: T, size: [usize; N]) -> Self {
        Self {
            size,
            matrix: vec![value; product(&size)].into_boxed_slice(),
        }
    }

    /// Fills the table with the given value.
    pub fn fill(&mut self, value: T) {
        self.matrix.fill(value);
    }
}

impl<T, const N: usize> TableNd<T, N> {
    /// Returns the multi-dimensional size of this table.
    pub fn size(&self) -> &[usize; N] {
        &self.size
    }
}

impl<T, const N: usize> TableNd<Option<T>, N> {
    /// Returns the capacity of this table, i.e. the total number of memory
    /// slots available.
    ///
    /// This is the product of the [`size()`](Self::size).
    pub fn capacity(&self) -> usize {
        product(&self.size)
    }

    /// Returns the number of values set to [`Some(_)`](Option::Some).
    pub fn len(&self) -> usize {
        self.matrix.iter().filter(|x| x.is_some()).count()
    }

    /// Checks if all the values are set to [`None`].
    pub fn is_empty(&self) -> bool {
        self.matrix.iter().all(|x| x.is_none())
    }
}

impl<T, const N: usize> Index<[usize; N]> for TableNd<T, N> {
    type Output = T;

    fn index(&self, index: [usize; N]) -> &Self::Output {
        &self.matrix[flatten_index(&index, &self.size)]
    }
}

impl<T, const N: usize> IndexMut<[usize; N]> for TableNd<T, N> {
    fn index_mut(&mut self, index: [usize; N]) -> &mut Self::Output {
        &mut self.matrix[flatten_index(&index, &self.size)]
    }
}

/// A sparse N-dimensional table.
pub struct SparseTableNd<T, const N: usize, S = RandomState> {
    size: [usize; N],
    matrix: HashMap<usize, T, S>,
}

/// A sparse 2-dimensional table.
pub type SparseTable2d<T, S = RandomState> = SparseTableNd<T, 2, S>;
/// A sparse 3-dimensional table.
pub type SparseTable3d<T, S = RandomState> = SparseTableNd<T, 3, S>;
/// A sparse 4-dimensional table.
pub type SparseTable4d<T, S = RandomState> = SparseTableNd<T, 4, S>;
/// A sparse 5-dimensional table.
pub type SparseTable5d<T, S = RandomState> = SparseTableNd<T, 5, S>;
/// A sparse 6-dimensional table.
pub type SparseTable6d<T, S = RandomState> = SparseTableNd<T, 6, S>;
/// A sparse 7-dimensional table.
pub type SparseTable7d<T, S = RandomState> = SparseTableNd<T, 7, S>;
/// A sparse 8-dimensional table.
pub type SparseTable8d<T, S = RandomState> = SparseTableNd<T, 8, S>;
/// A sparse 9-dimensional table.
pub type SparseTable9d<T, S = RandomState> = SparseTableNd<T, 9, S>;
/// A sparse 10-dimensional table.
pub type SparseTable10d<T, S = RandomState> = SparseTableNd<T, 10, S>;
/// A sparse 11-dimensional table.
pub type SparseTable11d<T, S = RandomState> = SparseTableNd<T, 11, S>;
/// A sparse 12-dimensional table.
pub type SparseTable12d<T, S = RandomState> = SparseTableNd<T, 12, S>;

impl<T, const N: usize> SparseTableNd<T, N, RandomState> {
    /// Creates a new sparse table of the given size.
    pub fn new(size: [usize; N]) -> Self {
        product(&size);
        Self {
            size,
            matrix: HashMap::new(),
        }
    }
}

impl<T, const N: usize, S> SparseTableNd<T, N, S> {
    /// Returns the multi-dimensional size of this table.
    pub fn size(&self) -> &[usize; N] {
        &self.size
    }

    /// Returns the capacity of this table.
    ///
    /// This is the product of the [`size()`](Self::size).
    pub fn capacity(&self) -> usize {
        product(&self.size)
    }

    /// Returns the number of values set in this table.
    pub fn len(&self) -> usize {
        self.matrix.len()
    }

    /// Checks if this table is empty.
    pub fn is_empty(&self) -> bool {
        self.matrix.is_empty()
    }
}

impl<T, const N: usize, S: BuildHasher> SparseTableNd<T, N, S> {
    /// Creates a new sparse table of the given size.
    pub fn with_hasher(size: [usize; N], hash_builder: S) -> Self {
        product(&size);
        Self {
            size,
            matrix: HashMap::with_hasher(hash_builder),
        }
    }

    /// Inserts a value at the given index, returning [`Some(_)`](Option::Some)
    /// if a previous value was set.
    pub fn insert(&mut self, index: [usize; N], value: T) -> Option<T> {
        self.matrix.insert(flatten_index(&index, &self.size), value)
    }

    /// Remove the value at the given index, returning [`Some(_)`](Option::Some)
    /// if a previous value was set.
    pub fn remove(&mut self, index: [usize; N]) -> Option<T> {
        self.matrix.remove(&flatten_index(&index, &self.size))
    }

    /// Retrieves the value at the given index.
    pub fn get(&self, index: [usize; N]) -> Option<&T> {
        self.matrix.get(&flatten_index(&index, &self.size))
    }

    /// Retrieves a reference to the value at the given index.
    pub fn get_mut(&mut self, index: [usize; N]) -> Option<&mut T> {
        self.matrix.get_mut(&flatten_index(&index, &self.size))
    }
}

#[track_caller]
fn product<const N: usize>(size: &[usize; N]) -> usize {
    size.iter().fold(1, |acc, x| acc.strict_mul(*x))
}

#[track_caller]
fn flatten_index<const N: usize>(index: &[usize; N], size: &[usize; N]) -> usize {
    for (i, &x) in index.iter().enumerate() {
        assert!(x < size[i], "{x} < {} for index[{i}]", size[i]);
    }

    let mut idx = 0;
    for (i, x) in index.iter().enumerate() {
        if i != 0 {
            idx *= size[i];
        }
        idx += x;
    }
    idx
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::Deref;

    #[test]
    fn table_init() {
        let table2d = Table2d::init(1, [2, 3]);
        assert_eq!(table2d[[0, 0]], 1);
        assert_eq!(table2d[[0, 1]], 1);
        assert_eq!(table2d[[0, 2]], 1);
        assert_eq!(table2d[[1, 0]], 1);
        assert_eq!(table2d[[1, 1]], 1);
        assert_eq!(table2d[[1, 2]], 1);
    }

    #[test]
    #[should_panic]
    fn table_init_overflow() {
        Table2d::init(1, [usize::MAX, usize::MAX]);
    }

    #[test]
    fn table_fill() {
        let mut table2d = Table2d::new([2, 3]);
        table2d.fill(7);
        assert_eq!(table2d[[0, 0]], 7);
        assert_eq!(table2d[[0, 1]], 7);
        assert_eq!(table2d[[0, 2]], 7);
        assert_eq!(table2d[[1, 0]], 7);
        assert_eq!(table2d[[1, 1]], 7);
        assert_eq!(table2d[[1, 2]], 7);
    }

    #[test]
    fn table_index() {
        let table2d = Table2d {
            size: [2, 4],
            matrix: vec![0, 1, 2, 3, 4, 5, 6, 7].into_boxed_slice(),
        };
        assert_eq!(table2d[[0, 0]], 0);
        assert_eq!(table2d[[0, 1]], 1);
        assert_eq!(table2d[[0, 2]], 2);
        assert_eq!(table2d[[0, 3]], 3);
        assert_eq!(table2d[[1, 0]], 4);
        assert_eq!(table2d[[1, 1]], 5);
        assert_eq!(table2d[[1, 2]], 6);
        assert_eq!(table2d[[1, 3]], 7);
    }

    #[test]
    fn table_index_mut() {
        let mut table2d = Table2d {
            size: [2, 4],
            matrix: vec![0, 1, 2, 3, 4, 5, 6, 7].into_boxed_slice(),
        };

        table2d[[1, 1]] = 9;
        assert_eq!(table2d.matrix.deref(), &[0, 1, 2, 3, 4, 9, 6, 7]);
    }
}
