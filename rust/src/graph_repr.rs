
use std::collections::{ BTreeMap, HashMap };

use linear_algebra::{
    matrix::Matrix, traits::Fillable, vector::Vector
};

pub trait GraphRepr{}

/// A static graph representation with fixed size.
///
/// This representation is useful for graphs with a fixed number of nodes.
///
pub struct StaticRepr<N, E, const SIZE: usize>
where
    N: 'static + Clone + Copy + Default + std::fmt::Debug,
    E: 'static + Clone + Copy + Default + std::fmt::Debug,
    [(); SIZE * SIZE]:
{
    pub(crate) nodes: Vector<Option<N>, SIZE>,
    pub(crate) edges: Matrix<Option<E>, SIZE, SIZE>
}

impl<N, E, const SIZE: usize> GraphRepr for StaticRepr<N, E, SIZE>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug,
    [(); SIZE * SIZE]:
{}

impl<N, E, const SIZE: usize> Default for StaticRepr<N, E, SIZE>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug,
    [(); SIZE * SIZE]: ,
{
    fn default() -> Self {
        Self {
            nodes: Vector::default(),
            edges: Matrix::default()
        }
    }
}

/// A dynamic graph representation with variable size.
///
/// This representation is useful for graphs with a variable number of nodes.
///
pub struct DynRepr<N, E> {
    pub(crate) data: Vec<(N, Vec<E>)>
}

impl<N, E> GraphRepr for DynRepr<N, E>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{}

impl<N, E> Default for DynRepr<N, E> {
    fn default() -> Self {
        Self {
            data: Vec::default()
        }
    }
}

/// A hash map graph representation.
///
/// This representation is useful for graphs with a variable number of nodes.
///
pub struct HashRepr<I, N, E> {
    pub(crate) data: HashMap<I, (N, HashMap<I, E>)>,
}

impl<I, N, E> GraphRepr for HashRepr<I, N, E>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{}

impl<I, N, E> Default for HashRepr<I, N, E>
where
    I: Ord + std::hash::Hash
{
    fn default() -> Self {
        Self {
            data: HashMap::default()
        }
    }
}

/// A B-tree map graph representation.
///
/// This representation is useful for graphs with a variable number of nodes.
///
pub struct BTreeRepr<I, N, E> {
    pub(crate) data: BTreeMap<I, (N, BTreeMap<I, E>)>,
}

impl<I, N, E> GraphRepr for BTreeRepr<I, N, E>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{}

impl<I, N, E> Default for BTreeRepr<I, N, E>
where
    I: Ord
{
    fn default() -> Self {
        Self {
            data: BTreeMap::default()
        }
    }
}
