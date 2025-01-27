
use std::{
    ops::{ Deref, DerefMut },
    collections::{ BTreeMap, HashMap }
};

use linear_algebra::{
    matrix::Matrix, traits::Fillable, vector::Vector
};

#[derive( Clone, Copy, Debug, Default )]
pub struct NodeRepr<N, A> {
    pub(crate) node: N,
    pub(crate) adjs: A
}

pub trait GraphRepr {}

/// A static graph representation with fixed size.
///
/// This representation is useful for graphs with a fixed number of nodes.
///
pub struct StaticRepr<N, E, const SIZE: usize>( pub(crate) [ NodeRepr<N, [ Option<E>; SIZE ]>; SIZE ] )
where
    N: 'static + Clone + Copy + Default + std::fmt::Debug,
    E: 'static + Clone + Copy + Default + std::fmt::Debug;

impl<N, E, const SIZE: usize> GraphRepr for StaticRepr<N, E, SIZE>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{}

impl<N, E, const SIZE: usize> Default for StaticRepr<N, E, SIZE>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug,
    [Option<E>; SIZE]: std::default::Default
{
    fn default() -> Self {
        Self ( [ NodeRepr::default(); SIZE ] )
    }
}

/// A dynamic graph representation with variable size.
///
/// This representation is useful for graphs with a variable number of nodes.
///
pub struct DynRepr<N, E> ( pub(crate) Vec<NodeRepr<N, Vec<E>>> );

impl<N, E> GraphRepr for DynRepr<N, E>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{}

impl<N, E> Default for DynRepr<N, E> {
    fn default() -> Self {
        Self ( Vec::default() )
    }
}

/// A hash map graph representation.
///
/// This representation is useful for graphs with a variable number of nodes.
///
pub struct HashRepr<I, N, E> ( pub(crate) HashMap<I, NodeRepr<N, HashMap<I, E>>> );

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
        Self ( HashMap::default() )
    }
}

/// A B-tree map graph representation.
///
/// This representation is useful for graphs with a variable number of nodes.
///
pub struct BTreeRepr<I, N, E> ( pub(crate) BTreeMap<I, NodeRepr<N, BTreeMap<I, E>>> );

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
        Self ( BTreeMap::default() )
    }
}
