
use std::collections::{ BTreeMap, HashMap };

use linear_algebra::{
    matrix::Matrix, traits::Fillable, vector::Vector
};

pub trait GraphRepr{}

pub trait GetNode<I, N> {
    fn node( &self, id: I ) -> Option<&N>;
}

pub trait GetNodeMut<I, N> {
    fn node_mut( &mut self, id: I ) -> Option<&mut N>;
}

pub trait GetEdge<I, E> {
    fn edge( &self, id1: I, id2: I ) -> Option<&E>;
}

pub trait GetEdgeMut<I, E> {
    fn edge_mut( &mut self, id1: I, id2: I ) -> Option<&mut E>;
}

pub trait AddNode<I, N> {
    fn add_node( &mut self, id: I, node: N );
}

pub trait RemoveNode<I, N> {
    fn remove_node( &mut self, id: I ) -> Option<N>;
}

pub trait AddEdge<I, E> {
    fn add_edge( &mut self, id1: I, id2: I, edge: E );
}

pub trait RemoveEdge<I, E> {
    fn remove_edge( &mut self, id1: I, id2: I ) -> Option<E>;
}

pub trait ContainsNode<I> {
    fn contains_node( &self, id: I ) -> bool;
}

pub trait ContainsEdge<I> {
    fn contains_edge( &self, id1: I, id2: I ) -> bool;
}

pub trait ClearNodes {
    fn clear_nodes( &mut self );
}

pub trait ClearEdges {
    fn clear_edges( &mut self );
}

pub trait IterNodes<'a, N>
where
    N: 'a
{
    fn iter_nodes( &'a self ) -> impl Iterator<Item = Option<&'a N>>;
}

pub trait IterNodesMut<'a, N>
where
    N: 'a
{
    fn iter_nodes_mut( &'a mut self ) -> impl Iterator<Item = Option<&'a mut N>>;
}

pub trait IterEdges<'a, I, E>
where
    E: 'a
{
    fn iter_edges( &'a self, id: I ) -> impl Iterator<Item = Option<&'a E>>;
}

pub trait IterEdgesMut<'a, I, E>
where
    E: 'a
{
    fn iter_edges_mut( &'a mut self, id: I ) -> impl Iterator<Item = Option<&'a mut E>>;
}

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
    nodes: Vector<Option<N>, SIZE>,
    edges: Matrix<Option<E>, SIZE, SIZE>
}

impl<N, E, const SIZE: usize> GraphRepr for StaticRepr<N, E, SIZE>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug,
    [(); SIZE * SIZE]:
{}

impl<N, E, const SIZE: usize> GetNode<usize, N> for StaticRepr<N, E, SIZE>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug,
    [(); SIZE * SIZE]:
{
    fn node( &self, id: usize ) -> Option<&N> {
        self.nodes[ id ].as_ref()
    }
}

impl<N, E, const SIZE: usize> GetNodeMut<usize, N> for StaticRepr<N, E, SIZE>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug,
    [(); SIZE * SIZE]:
{
    fn node_mut( &mut self, id: usize ) -> Option<&mut N> {
        self.nodes[ id ].as_mut()
    }
}

impl<N, E, const SIZE: usize> GetEdge<usize, E> for StaticRepr<N, E, SIZE>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug,
    [(); SIZE * SIZE]:
{
    fn edge( &self, id1: usize, id2: usize ) -> Option<&E> {
        self.edges[[ id1, id2 ]].as_ref()
    }
}

impl<N, E, const SIZE: usize> GetEdgeMut<usize, E> for StaticRepr<N, E, SIZE>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug,
    [(); SIZE * SIZE]:
{
    fn edge_mut( &mut self, id1: usize, id2: usize ) -> Option<&mut E> {
        self.edges[[ id1, id2 ]].as_mut()
    }
}

impl<N, E, const SIZE: usize> AddNode<usize, N> for StaticRepr<N, E, SIZE>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug,
    [(); SIZE * SIZE]:
{
    fn add_node( &mut self, id: usize, node: N ) {
        self.nodes[ id ] = Some( node );
    }
}

impl<N, E, const SIZE: usize> RemoveNode<usize, N> for StaticRepr<N, E, SIZE>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug,
    [(); SIZE * SIZE]:
{
    fn remove_node( &mut self, id: usize ) -> Option<N> {
        self.nodes[ id ].take()
    }
}

impl<N, E, const SIZE: usize> AddEdge<usize, E> for StaticRepr<N, E, SIZE>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug,
    [(); SIZE * SIZE]:
{
    fn add_edge( &mut self, id1: usize, id2: usize, edge: E ) {
        self.edges[[ id1, id2 ]] = Some( edge );
    }
}

impl<N, E, const SIZE: usize> RemoveEdge<usize, E> for StaticRepr<N, E, SIZE>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug,
    [(); SIZE * SIZE]:
{
    fn remove_edge( &mut self, id1: usize, id2: usize ) -> Option<E> {
        self.edges[[ id1, id2 ]].take()
    }
}

impl<N, E, const SIZE: usize> ContainsNode<usize> for StaticRepr<N, E, SIZE>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug,
    [(); SIZE * SIZE]:
{
    fn contains_node( &self, id: usize ) -> bool {
        self.nodes[ id ].is_some()
    }
}

impl<N, E, const SIZE: usize> ContainsEdge<usize> for StaticRepr<N, E, SIZE>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug,
    [(); SIZE * SIZE]:
{
    fn contains_edge( &self, id1: usize, id2: usize ) -> bool {
        self.edges[[ id1, id2 ]].is_some()
    }
}

impl<N, E, const SIZE: usize> ClearNodes for StaticRepr<N, E, SIZE>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug,
    [(); SIZE * SIZE]:
{
    fn clear_nodes( &mut self ) {
        self.nodes.fill( None );
    }
}

impl<N, E, const SIZE: usize> ClearEdges for StaticRepr<N, E, SIZE>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug,
    [(); SIZE * SIZE]:
{
    fn clear_edges( &mut self ) {
        self.edges.fill( None );
    }
}

impl<'a, N, E, const SIZE: usize> IterNodes<'a, N> for StaticRepr<N, E, SIZE>
where
    N: 'a + Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug,
    [(); SIZE * SIZE]:
{
    fn iter_nodes( &self ) -> impl Iterator<Item = Option<&N>> {
        self.nodes.iter().map( |node| node.as_ref() )
    }
}

impl<'a, N, E, const SIZE: usize> IterNodesMut<'a, N> for StaticRepr<N, E, SIZE>
where
    N: 'a + Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug,
    [(); SIZE * SIZE]:
{
    fn iter_nodes_mut( &mut self ) -> impl Iterator<Item = Option<&mut N>> {
        self.nodes.iter_mut().map( |node| node.as_mut() )
    }
}

impl<'a, N, E, const SIZE: usize> IterEdges<'a, usize, E> for StaticRepr<N, E, SIZE>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: 'a + Clone + Copy + Default + std::fmt::Debug,
    [(); SIZE * SIZE]:
{
    fn iter_edges( &self, id: usize ) -> impl Iterator<Item = Option<&E>> {
        self.edges.iter_col( id ).map( |edge| edge.as_ref() )
    }
}

impl<'a, N, E, const SIZE: usize> IterEdgesMut<'a, usize, E> for StaticRepr<N, E, SIZE>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: 'a + Clone + Copy + Default + std::fmt::Debug,
    [(); SIZE * SIZE]:
{
    fn iter_edges_mut( &mut self, id: usize ) -> impl Iterator<Item = Option<&mut E>> {
        self.edges.iter_col_mut( id ).map( |edge| edge.as_mut() )
    }
}

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
    data: Vec<(N, Vec<E>)>
}

impl<N, E> GraphRepr<usize, N, E> for DynRepr<N, E> {
    fn node( &self, id: usize ) -> Option<&N> {
        self.data.get( id ).map( |node| &node.0 )
    }

    fn node_mut( &mut self, id: usize ) -> Option<&mut N> {
        self.data.get_mut( id ).map( |node| &mut node.0 )
    }

    fn edge( &self, id1: usize, id2: usize ) -> Option<&E> {
        self.data.get( id1 ).and_then( |node| node.1.get( id2 ) )
    }

    fn edge_mut( &mut self, id1: usize, id2: usize ) -> Option<&mut E> {
        self.data.get_mut( id1 ).and_then( |node| node.1.get_mut( id2 ) )
    }

    fn add_node( &mut self, id: usize, node: N ) {
        self.data.insert( id, ( node, Vec::default() ) );
    }

    fn remove_node( &mut self, id: usize ) -> Option<N> {
        Some( self.data.remove( id ).0 )
    }

    fn add_edge( &mut self, id1: usize, id2: usize, edge: E ) {
        self.data.get_mut( id1 ).map( |node| node.1.insert( id2, edge ) );
    }

    fn remove_edge( &mut self, id1: usize, id2: usize ) -> Option<E> {
        self.data.get_mut( id1 ).and_then( |node| Some( node.1.remove( id2 ) ) )
    }

    fn contains_node( &self, id: usize ) -> bool {
        self.data.get( id ).is_some()
    }

    fn contains_edge( &self, id1: usize, id2: usize ) -> bool {
        self.data.get( id1 ).and_then( |node| node.1.get( id2 ) ).is_some()
    }

    fn clear_nodes( &mut self ) {
        self.data.clear();
    }

    fn clear_edges( &mut self ) {
        self.data.iter_mut().for_each( |node| node.1.clear() );
    }

    fn iter_nodes( &self ) -> impl Iterator<Item = Option<&N>> {
        self.data.iter().map( |node| Some( &node.0 ) )
    }

    fn iter_nodes_mut( &mut self ) -> impl Iterator<Item = Option<&mut N>> {
        self.data.iter_mut().map( |node| Some( &mut node.0 ) )
    }

    fn iter_edges( &self, id: usize ) -> impl Iterator<Item = Option<&E>> {
        self.data.get( id ).map( |node| node.1.iter().map( |edge| Some( edge ) ) ).into_iter().flatten()
    }

    fn iter_edges_mut( &mut self, id: usize ) -> impl Iterator<Item = Option<&mut E>> {
        self.data.get_mut( id ).map( |node| node.1.iter_mut().map( |edge| Some( edge ) ) ).into_iter().flatten()
    }
}

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
    data: HashMap<I, (N, HashMap<I, E>)>,
}

impl<I, N, E> GraphRepr<I, N, E> for HashRepr<I, N, E>
where
    I: Ord + std::hash::Hash
{
    fn node( &self, id: I ) -> Option<&N> {
        self.data.get( &id ).map( |node| &node.0 )
    }

    fn node_mut( &mut self, id: I ) -> Option<&mut N> {
        self.data.get_mut( &id ).map( |node| &mut node.0 )
    }

    fn edge( &self, id1: I, id2: I ) -> Option<&E> {
        self.data.get( &id1 ).and_then( |node| node.1.get( &id2 ) )
    }

    fn edge_mut( &mut self, id1: I, id2: I ) -> Option<&mut E> {
        self.data.get_mut( &id1 ).and_then( |node| node.1.get_mut( &id2 ) )
    }

    fn add_node( &mut self, id: I, node: N ) {
        self.data.insert( id, ( node, HashMap::default() ) );
    }

    fn remove_node( &mut self, id: I ) -> Option<N> {
        self.data.remove( &id ).map( |node| node.0 )
    }

    fn add_edge( &mut self, id1: I, id2: I, edge: E ) {
        self.data.get_mut( &id1 ).map( |node| node.1.insert( id2, edge ) );
    }

    fn remove_edge( &mut self, id1: I, id2: I ) -> Option<E> {
        self.data.get_mut( &id1 ).and_then( |node| node.1.remove( &id2 ) )
    }

    fn contains_node( &self, id: I ) -> bool {
        self.data.contains_key( &id )
    }

    fn contains_edge( &self, id1: I, id2: I ) -> bool {
        self.data.get( &id1 ).is_some_and( |node| node.1.contains_key( &id2 ) )
    }

    fn clear_nodes( &mut self ) {
        self.data.clear();
    }

    fn clear_edges( &mut self ) {
        self.data.iter_mut().for_each( |node| node.1.1.clear() );
    }

    fn iter_nodes( &self ) -> impl Iterator<Item = Option<&N>> {
        self.data.iter().map( |node| Some( &node.1.0 ) )
    }

    fn iter_nodes_mut( &mut self ) -> impl Iterator<Item = Option<&mut N>> {
        self.data.iter_mut().map( |node| Some( &mut node.1.0 ) )
    }

    fn iter_edges( &self, id: I ) -> impl Iterator<Item = Option<&E>> {
        self.data.get( &id ).map( |node| node.1.iter().map( |edge| Some( edge.1 ) ) ).into_iter().flatten()
    }

    fn iter_edges_mut( &mut self, id: I ) -> impl Iterator<Item = Option<&mut E>> {
        self.data.get_mut( &id ).map( |node| node.1.iter_mut().map( |edge| Some( edge.1 ) ) ).into_iter().flatten()
    }
}

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
    data: BTreeMap<I, (N, BTreeMap<I, E>)>,
}

impl<I, N, E> GraphRepr<I, N, E> for BTreeRepr<I, N, E>
where
    I: Ord
{
    fn node( &self, id: I ) -> Option<&N> {
        self.data.get( &id ).map( |node| &node.0 )
    }

    fn node_mut( &mut self, id: I ) -> Option<&mut N> {
        self.data.get_mut( &id ).map( |node| &mut node.0 )
    }

    fn edge( &self, id1: I, id2: I ) -> Option<&E> {
        self.data.get( &id1 ).and_then( |node| node.1.get( &id2 ) )
    }

    fn edge_mut( &mut self, id1: I, id2: I ) -> Option<&mut E> {
        self.data.get_mut( &id1 ).and_then( |node| node.1.get_mut( &id2 ) )
    }

    fn add_node( &mut self, id: I, node: N ) {
        self.data.insert( id, ( node, BTreeMap::default() ) );
    }

    fn remove_node( &mut self, id: I ) -> Option<N> {
        self.data.remove( &id ).map( |node| node.0 )
    }

    fn add_edge( &mut self, id1: I, id2: I, edge: E ) {
        self.data.get_mut( &id1 ).map( |node| node.1.insert( id2, edge ) );
    }

    fn remove_edge( &mut self, id1: I, id2: I ) -> Option<E> {
        self.data.get_mut( &id1 ).and_then( |node| node.1.remove( &id2 ) )
    }

    fn contains_node( &self, id: I ) -> bool {
        self.data.contains_key( &id )
    }

    fn contains_edge( &self, id1: I, id2: I ) -> bool {
        self.data.get( &id1 ).is_some_and( |node| node.1.contains_key( &id2 ) )
    }

    fn clear_nodes( &mut self ) {
        self.data.clear();
    }

    fn clear_edges( &mut self ) {
        self.data.iter_mut().for_each( |node| node.1.1.clear() );
    }

    fn iter_nodes( &self ) -> impl Iterator<Item = Option<&N>> {
        self.data.iter().map( |node| Some( &node.1.0 ) )
    }

    fn iter_nodes_mut( &mut self ) -> impl Iterator<Item = Option<&mut N>> {
        self.data.iter_mut().map( |node| Some( &mut node.1.0 ) )
    }

    fn iter_edges( &self, id: I ) -> impl Iterator<Item = Option<&E>> {
        self.data.get( &id ).map( |node| node.1.iter().map( |edge| Some( edge.1 ) ) ).into_iter().flatten()
    }

    fn iter_edges_mut( &mut self, id: I ) -> impl Iterator<Item = Option<&mut E>> {
        self.data.get_mut( &id ).map( |node| node.1.iter_mut().map( |edge| Some( edge.1 ) ) ).into_iter().flatten()
    }
}

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
