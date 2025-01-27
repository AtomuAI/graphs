// Copyright 2024 Bewusstsein Labs

//: Standard
use std::{
    cmp::{ Eq, Ord, PartialEq },
    collections::{ BTreeMap, HashMap },
    marker::PhantomData,
    ops::{ Deref, DerefMut, Not }
};

use crate::{
    graph::Graph,
    graph_repr::DynRepr,
    traits::{
        GetNode,
        GetNodeMut,
        GetEdge,
        GetEdgeMut,
        AddNode,
        RemoveNode,
        AddEdge,
        RemoveEdge,
        ContainsNode,
        ContainsEdge,
        ClearNodes,
        ClearEdges,
        IterNodes,
        IterNodesMut,
        IterEdges,
        IterEdgesMut,
        IsComplete,
        IsEmpty,
        IsTrivial,
        IsNull,
        IsChildNode,
        IsSubgraph,
        IsProperSubgraph,
        IsImproperSubgraph,
        IsSpanningSubgraph,
        AreAdjacentNodes,
        AreAdjacentEdges,
        Order,
        Size
    }
};

impl<N, E> GetNode<usize, N> for Graph<DynRepr<N, E>>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn node( &self, id: usize ) -> Option<&N> {
        self.0.nodes[ id ].as_ref()
    }
}

impl<N, E> GetNodeMut<usize, N> for Graph<DynRepr<N, E>>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn node_mut( &mut self, id: usize ) -> Option<&mut N> {
        self.0.nodes[ id ].as_mut()
    }
}

impl<N, E> GetEdge<usize, E> for Graph<DynRepr<N, E>>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn edge( &self, id1: usize, id2: usize ) -> Option<&E> {
        self.0.edges[[ id1, id2 ]].as_ref()
    }
}

impl<N, E> GetEdgeMut<usize, E> for Graph<DynRepr<N, E>>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn edge_mut( &mut self, id1: usize, id2: usize ) -> Option<&mut E> {
        self.0.edges[[ id1, id2 ]].as_mut()
    }
}

impl<N, E> AddNode<usize, N> for Graph<DynRepr<N, E>>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn add_node( &mut self, id: usize, node: N ) {
        self.0.nodes[ id ] = Some( node );
    }
}

impl<N, E> RemoveNode<usize, N> for Graph<DynRepr<N, E>>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn remove_node( &mut self, id: usize ) -> Option<N> {
        self.0.nodes[ id ].take()
    }
}

impl<N, E> AddEdge<usize, E> for Graph<DynRepr<N, E>>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn add_edge( &mut self, id1: usize, id2: usize, edge: E ) {
        self.0.edges[[ id1, id2 ]] = Some( edge );
    }
}

impl<N, E> RemoveEdge<usize, E> for Graph<DynRepr<N, E>>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn remove_edge( &mut self, id1: usize, id2: usize ) -> Option<E> {
        self.0.edges[[ id1, id2 ]].take()
    }
}

impl<N, E> ContainsNode<usize> for Graph<DynRepr<N, E>>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn contains_node( &self, id: usize ) -> bool {
        self.0.nodes[ id ].is_some()
    }
}

impl<N, E> ContainsEdge<usize> for Graph<DynRepr<N, E>>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn contains_edge( &self, id1: usize, id2: usize ) -> bool {
        self.0.edges[[ id1, id2 ]].is_some()
    }
}

impl<N, E> ClearNodes for Graph<DynRepr<N, E>>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn clear_nodes( &mut self ) {
        self.0.nodes.fill( None );
    }
}

impl<N, E> ClearEdges for Graph<DynRepr<N, E>>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn clear_edges( &mut self ) {
        self.0.edges.fill( None );
    }
}

impl<'a, N, E> IterNodes<'a, N> for Graph<DynRepr<N, E>>
where
    N: 'a + Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn iter_nodes( &self ) -> impl Iterator<Item = Option<&N>> {
        self.0.nodes.iter().map( |node| node.as_ref() )
    }
}

impl<'a, N, E> IterNodesMut<'a, N> for Graph<DynRepr<N, E>>
where
    N: 'a + Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn iter_nodes_mut( &mut self ) -> impl Iterator<Item = Option<&mut N>> {
        self.0.nodes.iter_mut().map( |node| node.as_mut() )
    }
}

impl<'a, N, E> IterEdges<'a, usize, E> for Graph<DynRepr<N, E>>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: 'a + Clone + Copy + Default + std::fmt::Debug
{
    fn iter_edges( &self, id: usize ) -> impl Iterator<Item = Option<&E>> {
        self.0.edges.iter_col( id ).map( |edge| edge.as_ref() )
    }
}

impl<'a, N, E> IterEdgesMut<'a, usize, E> for Graph<DynRepr<N, E>>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: 'a + Clone + Copy + Default + std::fmt::Debug
{
    fn iter_edges_mut( &mut self, id: usize ) -> impl Iterator<Item = Option<&mut E>> {
        self.0.edges.iter_col_mut( id ).map( |edge| edge.as_mut() )
    }
}

impl<N, E> IsComplete<usize, N, E> for Graph<DynRepr<N, E>>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn is_complete( &self ) -> bool {
        for ( node, neighbors ) in self.0.data.iter() {
            if neighbors.adjacencies().len() != self.0.data.len() - 1 {
                return false;
            }
            for neighbor in neighbors.adjacencies().keys() {
                if !self.0.data.node( neighbor ).is_some_and( |n| n.adjacencies().contains_key( node ) ) {
                    return false;
                }
            }
        }
        true
    }
}

impl<N, E> IsEmpty<usize, N, E> for Graph<DynRepr<N, E>>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn is_empty( &self ) -> bool {
        self.0.data.values().all( |neighbors| neighbors.adjacencies().is_empty() )
    }
}

impl<N, E> IsTrivial<usize, N, E> for Graph<DynRepr<N, E>>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn is_trivial( &self ) -> bool {
        self.0.data.len() == 1 && self.data().values().next().is_some_and( |neighbors| neighbors.adjacencies().is_empty())
    }
}

impl<'a, N, E> IsNull<usize, N, E> for Graph<DynRepr<N, E>>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn is_null( &self ) -> bool {
        self.0.data.is_empty()
    }
}

impl<N, E> IsChildNode<usize, N, E> for Graph<DynRepr<N, E>>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn is_child_node( &self, node_1: usize ) -> bool {
        self.0.data.contains_node( node_1 )
    }
}

impl<N, E> IsSubgraph<usize, N, E> for Graph<DynRepr<N, E>>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn is_subgraph(&self, subgraph: &Self) -> bool {
        subgraph.0.data.iter().all( |(node, neighbors)| {
            self.0.data.get( node ).is_some_and( |graph_node| {
                neighbors.adjacencies().keys().all( |key| graph_node.adjacencies().contains_key( key ) )
            })
        })
    }
}

impl<N, E> IsProperSubgraph<usize, N, E> for Graph<DynRepr<N, E>>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn is_proper_subgraph( &self, subgraph: &Self ) -> bool {
        self.0.data != subgraph.0.data && self.is_subgraph( subgraph )
    }
}

impl<N, E> IsImproperSubgraph<usize, N, E> for Graph<DynRepr<N, E>>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn is_improper_subgraph( &self, subgraph: &Self ) -> bool {
        self.0.data == subgraph.0.data
    }
}

impl<N, E> IsSpanningSubgraph<usize, N, E> for Graph<DynRepr<N, E>>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn is_spanning_subgraph( &self, subgraph: &Self ) -> bool {
        self.0.data.len() == subgraph.data().len() && self.is_subgraph( subgraph )
    }
}

impl<N, E> AreAdjacentNodes<usize, N, E> for Graph<DynRepr<N, E>>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn are_adjacent_nodes( &self, node_1: usize, node_2: usize ) -> bool {
        self.is_child_node( node_1.clone() )
            && self.is_child_node( node_2.clone() )
            && self.0.data.get( &node_1 ).unwrap().adjacencies().contains_key( &node_2 )
    }
}

impl<N, E> AreAdjacentEdges<usize, N, E> for Graph<DynRepr<N, E>>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn are_adjacent_edges( &self, node_1: usize, node_2: usize, node_3: usize ) -> bool {
        self.are_adjacent_nodes( node_1, node_2.clone() )
            && self.are_adjacent_nodes( node_2, node_3 )
    }
}

impl<N, E> Order<usize, N, E> for Graph<DynRepr<N, E>>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn order( &self ) -> usize {
        self.0.data.len()
    }
}

impl<N, E> Size<usize, N, E> for Graph<DynRepr<N, E>>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn size( &self ) -> usize {
        self.0.data.values().map( |neighbors| neighbors.adjacencies().len() ).sum::<usize>() / 2
    }
}
