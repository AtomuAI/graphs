// Copyright 2024 Bewusstsein Labs

#![warn(private_bounds)]

//pub mod traverser;

//: Standard
use std::{
    cmp::{ Eq, Ord, PartialEq },
    collections::{ HashMap, BTreeMap },
    marker::PhantomData,
    ops::{ Deref, DerefMut }
};

use crate::{
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
    },
    graph_repr::{
        GraphRepr,
        StaticRepr,
        DynRepr,
        HashRepr,
        BTreeRepr
    }
};

#[derive( Debug, Clone, Default, PartialEq, Eq )]
pub struct Graph<Repr>( Repr )
where
    Repr: GraphRepr;

impl<N, E, const SIZE: usize> GetNode<usize, N> for Graph<StaticRepr<N, E, SIZE>>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug,
    [(); SIZE * SIZE]:
{
    fn node( &self, id: usize ) -> Option<&N> {
        self.0.nodes[ id ].as_ref()
    }
}

impl<N, E, const SIZE: usize> GetNodeMut<usize, N> for Graph<StaticRepr<N, E, SIZE>>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug,
    [(); SIZE * SIZE]:
{
    fn node_mut( &mut self, id: usize ) -> Option<&mut N> {
        self.0.nodes[ id ].as_mut()
    }
}

impl<N, E, const SIZE: usize> GetEdge<usize, E> for Graph<StaticRepr<N, E, SIZE>>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug,
    [(); SIZE * SIZE]:
{
    fn edge( &self, id1: usize, id2: usize ) -> Option<&E> {
        self.0.edges[[ id1, id2 ]].as_ref()
    }
}

impl<N, E, const SIZE: usize> GetEdgeMut<usize, E> for Graph<StaticRepr<N, E, SIZE>>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug,
    [(); SIZE * SIZE]:
{
    fn edge_mut( &mut self, id1: usize, id2: usize ) -> Option<&mut E> {
        self.0.edges[[ id1, id2 ]].as_mut()
    }
}

impl<N, E, const SIZE: usize> AddNode<usize, N> for Graph<StaticRepr<N, E, SIZE>>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug,
    [(); SIZE * SIZE]:
{
    fn add_node( &mut self, id: usize, node: N ) {
        self.0.nodes[ id ] = Some( node );
    }
}

impl<N, E, const SIZE: usize> RemoveNode<usize, N> for Graph<StaticRepr<N, E, SIZE>>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug,
    [(); SIZE * SIZE]:
{
    fn remove_node( &mut self, id: usize ) -> Option<N> {
        self.0.nodes[ id ].take()
    }
}

impl<N, E, const SIZE: usize> AddEdge<usize, E> for Graph<StaticRepr<N, E, SIZE>>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug,
    [(); SIZE * SIZE]:
{
    fn add_edge( &mut self, id1: usize, id2: usize, edge: E ) {
        self.0.edges[[ id1, id2 ]] = Some( edge );
    }
}

impl<N, E, const SIZE: usize> RemoveEdge<usize, E> for Graph<StaticRepr<N, E, SIZE>>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug,
    [(); SIZE * SIZE]:
{
    fn remove_edge( &mut self, id1: usize, id2: usize ) -> Option<E> {
        self.0.edges[[ id1, id2 ]].take()
    }
}

impl<N, E, const SIZE: usize> ContainsNode<usize> for Graph<StaticRepr<N, E, SIZE>>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug,
    [(); SIZE * SIZE]:
{
    fn contains_node( &self, id: usize ) -> bool {
        self.0.nodes[ id ].is_some()
    }
}

impl<N, E, const SIZE: usize> ContainsEdge<usize> for Graph<StaticRepr<N, E, SIZE>>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug,
    [(); SIZE * SIZE]:
{
    fn contains_edge( &self, id1: usize, id2: usize ) -> bool {
        self.0.edges[[ id1, id2 ]].is_some()
    }
}

impl<N, E, const SIZE: usize> ClearNodes for Graph<StaticRepr<N, E, SIZE>>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug,
    [(); SIZE * SIZE]:
{
    fn clear_nodes( &mut self ) {
        self.0.nodes.fill( None );
    }
}

impl<N, E, const SIZE: usize> ClearEdges for Graph<StaticRepr<N, E, SIZE>>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug,
    [(); SIZE * SIZE]:
{
    fn clear_edges( &mut self ) {
        self.0.edges.fill( None );
    }
}

impl<'a, N, E, const SIZE: usize> IterNodes<'a, N> for Graph<StaticRepr<N, E, SIZE>>
where
    N: 'a + Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug,
    [(); SIZE * SIZE]:
{
    fn iter_nodes( &self ) -> impl Iterator<Item = Option<&N>> {
        self.0.nodes.iter().map( |node| node.as_ref() )
    }
}

impl<'a, N, E, const SIZE: usize> IterNodesMut<'a, N> for Graph<StaticRepr<N, E, SIZE>>
where
    N: 'a + Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug,
    [(); SIZE * SIZE]:
{
    fn iter_nodes_mut( &mut self ) -> impl Iterator<Item = Option<&mut N>> {
        self.0.nodes.iter_mut().map( |node| node.as_mut() )
    }
}

impl<'a, N, E, const SIZE: usize> IterEdges<'a, usize, E> for Graph<StaticRepr<N, E, SIZE>>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: 'a + Clone + Copy + Default + std::fmt::Debug,
    [(); SIZE * SIZE]:
{
    fn iter_edges( &self, id: &usize ) -> impl Iterator<Item = Option<&E>> {
        self.0.edges.iter_col( *id ).map( |edge| edge.as_ref() )
    }
}

impl<'a, N, E, const SIZE: usize> IterEdgesMut<'a, usize, E> for Graph<StaticRepr<N, E, SIZE>>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: 'a + Clone + Copy + Default + std::fmt::Debug,
    [(); SIZE * SIZE]:
{
    fn iter_edges_mut( &mut self, id: &usize ) -> impl Iterator<Item = Option<&mut E>> {
        self.0.edges.iter_col_mut( *id ).map( |edge| edge.as_mut() )
    }
}

impl<N, E, const SIZE: usize> IsComplete<usize, N, E> for Graph<StaticRepr<N, E, SIZE>>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug,
    [(); SIZE * SIZE]:
{
    fn is_complete( &self ) -> bool {
        self.0.edges.iter().find( |edge| edge.is_none() ).is_some()
    }
}

impl<N, E, const SIZE: usize> IsEmpty<usize, N, E> for Graph<StaticRepr<N, E, SIZE>>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug,
    [(); SIZE * SIZE]:
{
    default fn is_empty( &self ) -> bool {
        self.0.edges.iter().find( |edge| edge.is_some() ).is_none()
    }
}

impl<N, E, const SIZE: usize> IsTrivial<usize, N, E> for Graph<StaticRepr<N, E, SIZE>>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug,
    [(); SIZE * SIZE]:
{
    default fn is_trivial( &self ) -> bool {
        self.0.data.len() == 1 && self.data().values().next().is_some_and( |neighbors| neighbors.adjacencies().is_empty())
    }
}

impl<N, E, const SIZE: usize> IsNull<usize, N, E> for Graph<StaticRepr<N, E, SIZE>>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug,
    [(); SIZE * SIZE]:
{
    default fn is_null( &self ) -> bool {
        self.0.data.is_empty()
    }
}

impl<N, E, const SIZE: usize> IsChildNode<usize, N, E> for Graph<StaticRepr<N, E, SIZE>>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug,
    [(); SIZE * SIZE]:
{
    default fn is_child_node( &self, node_1: usize ) -> bool {
        self.0.data.contains_node( node_1 )
    }
}

impl<N, E, const SIZE: usize> IsSubgraph<usize, N, E> for Graph<StaticRepr<N, E, SIZE>>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug,
    [(); SIZE * SIZE]:
{
    default fn is_subgraph(&self, subgraph: &Self) -> bool {
        subgraph.0.data.iter().all( |(node, neighbors)| {
            self.0.data.get( node ).is_some_and( |graph_node| {
                neighbors.adjacencies().keys().all( |key| graph_node.adjacencies().contains_key( key ) )
            })
        })
    }
}

impl<N, E, const SIZE: usize> IsProperSubgraph<usize, N, E> for Graph<StaticRepr<N, E, SIZE>>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug,
    [(); SIZE * SIZE]:
{
    default fn is_proper_subgraph( &self, subgraph: &Self ) -> bool {
        self.0.data != subgraph.0.data && self.is_subgraph( subgraph )
    }
}

impl<N, E, const SIZE: usize> IsImproperSubgraph<usize, N, E> for Graph<StaticRepr<N, E, SIZE>>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug,
    [(); SIZE * SIZE]:
{
    default fn is_improper_subgraph( &self, subgraph: &Self ) -> bool {
        self.0.data == subgraph.0.data
    }
}

impl<N, E, const SIZE: usize> IsSpanningSubgraph<usize, N, E> for Graph<StaticRepr<N, E, SIZE>>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug,
    [(); SIZE * SIZE]:
{
    default fn is_spanning_subgraph( &self, subgraph: &Self ) -> bool {
        self.0.data.len() == subgraph.data().len() && self.is_subgraph( subgraph )
    }
}

impl<N, E, const SIZE: usize> AreAdjacentNodes<usize, N, E> for Graph<StaticRepr<N, E, SIZE>>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug,
    [(); SIZE * SIZE]:
{
    default fn are_adjacent_nodes( &self, node_1: usize, node_2: usize ) -> bool {
        self.is_child_node( node_1.clone() )
            && self.is_child_node( node_2.clone() )
            && self.0.data.get( &node_1 ).unwrap().adjacencies().contains_key( &node_2 )
    }
}

impl<N, E, const SIZE: usize> AreAdjacentEdges<usize, N, E> for Graph<StaticRepr<N, E, SIZE>>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug,
    [(); SIZE * SIZE]:
{
    default fn are_adjacent_edges( &self, node_1: usize, node_2: usize, node_3: usize ) -> bool {
        self.are_adjacent_nodes( node_1, node_2.clone() )
            && self.are_adjacent_nodes( node_2, node_3 )
    }
}

impl<N, E, const SIZE: usize> Order<usize, N, E> for Graph<StaticRepr<N, E, SIZE>>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug,
    [(); SIZE * SIZE]:
{
    default fn order( &self ) -> usize {
        self.0.data.len()
    }
}

impl<N, E, const SIZE: usize> Size<usize, N, E> for Graph<StaticRepr<N, E, SIZE>>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug,
    [(); SIZE * SIZE]:
{
    default fn size( &self ) -> usize {
        self.0.data.values().map( |neighbors| neighbors.adjacencies().len() ).sum::<usize>() / 2
    }
}

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
    default fn is_empty( &self ) -> bool {
        self.0.data.values().all( |neighbors| neighbors.adjacencies().is_empty() )
    }
}

impl<N, E> IsTrivial<usize, N, E> for Graph<DynRepr<N, E>>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    default fn is_trivial( &self ) -> bool {
        self.0.data.len() == 1 && self.data().values().next().is_some_and( |neighbors| neighbors.adjacencies().is_empty())
    }
}

impl<'a, N, E> IsNull<usize, N, E> for Graph<DynRepr<N, E>>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    default fn is_null( &self ) -> bool {
        self.0.data.is_empty()
    }
}

impl<N, E> IsChildNode<usize, N, E> for Graph<DynRepr<N, E>>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    default fn is_child_node( &self, node_1: usize ) -> bool {
        self.0.data.contains_node( node_1 )
    }
}

impl<N, E> IsSubgraph<usize, N, E> for Graph<DynRepr<N, E>>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    default fn is_subgraph(&self, subgraph: &Self) -> bool {
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
    default fn is_proper_subgraph( &self, subgraph: &Self ) -> bool {
        self.0.data != subgraph.0.data && self.is_subgraph( subgraph )
    }
}

impl<N, E> IsImproperSubgraph<usize, N, E> for Graph<DynRepr<N, E>>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    default fn is_improper_subgraph( &self, subgraph: &Self ) -> bool {
        self.0.data == subgraph.0.data
    }
}

impl<N, E> IsSpanningSubgraph<usize, N, E> for Graph<DynRepr<N, E>>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    default fn is_spanning_subgraph( &self, subgraph: &Self ) -> bool {
        self.0.data.len() == subgraph.data().len() && self.is_subgraph( subgraph )
    }
}

impl<N, E> AreAdjacentNodes<usize, N, E> for Graph<DynRepr<N, E>>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    default fn are_adjacent_nodes( &self, node_1: usize, node_2: usize ) -> bool {
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
    default fn are_adjacent_edges( &self, node_1: usize, node_2: usize, node_3: usize ) -> bool {
        self.are_adjacent_nodes( node_1, node_2.clone() )
            && self.are_adjacent_nodes( node_2, node_3 )
    }
}

impl<N, E> Order<usize, N, E> for Graph<DynRepr<N, E>>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    default fn order( &self ) -> usize {
        self.0.data.len()
    }
}

impl<N, E> Size<usize, N, E> for Graph<DynRepr<N, E>>
where
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    default fn size( &self ) -> usize {
        self.0.data.values().map( |neighbors| neighbors.adjacencies().len() ).sum::<usize>() / 2
    }
}

impl<I, N, E> GetNode<I, N> for Graph<HashRepr<I, N, E>>
where
    I: Ord + std::hash::Hash,
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn node( &self, id: I ) -> Option<&N> {
        self.0.data.get( &id ).map( |node| &node.0 )
    }
}

impl<I, N, E> GetNodeMut<I, N> for Graph<HashRepr<I, N, E>>
where
    I: Ord + std::hash::Hash,
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn node_mut( &mut self, id: I ) -> Option<&mut N> {
        self.0.data.get_mut( &id ).map( |node| &mut node.0 )
    }
}

impl<I, N, E> GetEdge<I, E> for Graph<HashRepr<I, N, E>>
where
    I: Ord + std::hash::Hash,
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn edge( &self, id1: I, id2: I ) -> Option<&E> {
        self.0.data.get( &id1 ).and_then( |node| node.1.get( &id2 ) )
    }
}

impl<I, N, E> GetEdgeMut<I, E> for Graph<HashRepr<I, N, E>>
where
    I: Ord + std::hash::Hash,
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn edge_mut( &mut self, id1: I, id2: I ) -> Option<&mut E> {
        self.0.data.get_mut( &id1 ).and_then( |node| node.1.get_mut( &id2 ) )
    }
}

impl<I, N, E> AddNode<I, N> for Graph<HashRepr<I, N, E>>
where
    I: Ord + std::hash::Hash,
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn add_node( &mut self, id: I, node: N ) {
        self.0.data.insert( id, ( node, HashMap::default() ) );
    }
}

impl<I, N, E> RemoveNode<I, N> for Graph<HashRepr<I, N, E>>
where
    I: Ord + std::hash::Hash,
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn remove_node( &mut self, id: I ) -> Option<N> {
        self.0.data.remove( &id ).map( |node| node.0 )
    }
}

impl<I, N, E> AddEdge<I, E> for Graph<HashRepr<I, N, E>>
where
    I: Ord + std::hash::Hash,
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn add_edge( &mut self, id1: I, id2: I, edge: E ) {
        self.0.data.get_mut( &id1 ).map( |node| node.1.insert( id2, edge ) );
    }
}

impl<I, N, E> RemoveEdge<I, E> for Graph<HashRepr<I, N, E>>
where
    I: Ord + std::hash::Hash,
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn remove_edge( &mut self, id1: I, id2: I ) -> Option<E> {
        self.0.data.get_mut( &id1 ).and_then( |node| node.1.remove( &id2 ) )
    }
}

impl<I, N, E> ContainsNode<I> for Graph<HashRepr<I, N, E>>
where
    I: Ord + std::hash::Hash,
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn contains_node( &self, id: I ) -> bool {
        self.0.data.contains_key( &id )
    }
}

impl<I, N, E> ContainsEdge<I> for Graph<HashRepr<I, N, E>>
where
    I: Ord + std::hash::Hash,
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn contains_edge( &self, id1: I, id2: I ) -> bool {
        self.0.data.get( &id1 ).is_some_and( |node| node.1.contains_key( &id2 ) )
    }
}

impl<I, N, E> ClearNodes for Graph<HashRepr<I, N, E>>
where
    I: Ord + std::hash::Hash,
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn clear_nodes( &mut self ) {
        self.0.data.clear();
    }
}

impl<I, N, E> ClearEdges for Graph<HashRepr<I, N, E>>
where
    I: Ord + std::hash::Hash,
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn clear_edges( &mut self ) {
        self.0.data.iter_mut().for_each( |node| node.1.1.clear() );
    }
}

impl<'a, I, N, E> IterNodes<'a, N> for Graph<HashRepr<I, N, E>>
where
    I: Ord + std::hash::Hash,
    N: 'a + Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn iter_nodes( &self ) -> impl Iterator<Item = Option<&N>> {
        self.0.data.iter().map( |node| Some( &node.1.0 ) )
    }
}

impl<'a, I, N, E> IterNodesMut<'a, N> for Graph<HashRepr<I, N, E>>
where
    I: Ord + std::hash::Hash,
    N: 'a + Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn iter_nodes_mut( &mut self ) -> impl Iterator<Item = Option<&mut N>> {
        self.0.data.iter_mut().map( |node| Some( &mut node.1.0 ) )
    }
}

impl<'a, I, N, E> IterEdges<'a, I, E> for Graph<HashRepr<I, N, E>>
where
    I: Ord + std::hash::Hash,
    N: Clone + Copy + Default + std::fmt::Debug,
    E: 'a + Clone + Copy + Default + std::fmt::Debug
{
    fn iter_edges( &self, id: I ) -> impl Iterator<Item = Option<&E>> {
        self.0.data.get( &id ).map( |node| node.1.iter().map( |edge| Some( edge.1 ) ) ).into_iter().flatten()
    }
}

impl<'a, I, N, E> IterEdgesMut<'a, I, E> for Graph<HashRepr<I, N, E>>
where
    I: Ord + std::hash::Hash,
    N: Clone + Copy + Default + std::fmt::Debug,
    E: 'a + Clone + Copy + Default + std::fmt::Debug
{
    fn iter_edges_mut( &mut self, id: I ) -> impl Iterator<Item = Option<&mut E>> {
        self.0.data.get_mut( &id ).map( |node| node.1.iter_mut().map( |edge| Some( edge.1 ) ) ).into_iter().flatten()
    }
}

impl<I, N, E> IsComplete<I, N, E> for Graph<HashRepr<I, N, E>>
where
    I: Clone + Ord + std::hash::Hash,
    N: Copy + Default + PartialEq + std::fmt::Debug,
    E: Copy + Default + PartialEq + std::fmt::Debug
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

impl<I, N, E> IsEmpty<I, N, E> for Graph<HashRepr<I, N, E>>
where
    I: Clone + Ord + std::hash::Hash,
    N: Copy + Default + PartialEq + std::fmt::Debug,
    E: Copy + Default + PartialEq + std::fmt::Debug
{
    default fn is_empty( &self ) -> bool {
        self.0.data.values().all( |neighbors| neighbors.adjacencies().is_empty() )
    }
}

impl<I, N, E> IsTrivial<I, N, E> for Graph<HashRepr<I, N, E>>
where
    I: Clone + Ord + std::hash::Hash,
    N: Copy + Default + PartialEq + std::fmt::Debug,
    E: Copy + Default + PartialEq + std::fmt::Debug
{
    default fn is_trivial( &self ) -> bool {
        self.0.data.len() == 1 && self.data().values().next().is_some_and( |neighbors| neighbors.adjacencies().is_empty())
    }
}

impl<I, N, E> IsNull<I, N, E> for Graph<HashRepr<I, N, E>>
where
    I: Clone + Ord + std::hash::Hash,
    N: Copy + Default + PartialEq + std::fmt::Debug,
    E: Copy + Default + PartialEq + std::fmt::Debug
{
    default fn is_null( &self ) -> bool {
        self.0.data.is_empty()
    }
}

impl<I, N, E> IsChildNode<I, N, E> for Graph<HashRepr<I, N, E>>
where
    I: Clone + Ord + std::hash::Hash,
    N: Copy + Default + PartialEq + std::fmt::Debug,
    E: Copy + Default + PartialEq + std::fmt::Debug
{
    default fn is_child_node( &self, node_1: I ) -> bool {
        self.0.data.contains_node( node_1 )
    }
}

impl<I, N, E> IsSubgraph<I, N, E> for Graph<HashRepr<I, N, E>>
where
    I: Clone + Ord + std::hash::Hash,
    N: Copy + Default + PartialEq + std::fmt::Debug,
    E: Copy + Default + PartialEq + std::fmt::Debug
{
    default fn is_subgraph(&self, subgraph: &Self) -> bool {
        subgraph.0.data.iter().all( |(node, neighbors)| {
            self.0.data.get( node ).is_some_and( |graph_node| {
                neighbors.adjacencies().keys().all( |key| graph_node.adjacencies().contains_key( key ) )
            })
        })
    }
}

impl<I, N, E> IsProperSubgraph<I, N, E> for Graph<HashRepr<I, N, E>>
where
    I: Clone + Ord + std::hash::Hash,
    N: Copy + Default + PartialEq + std::fmt::Debug,
    E: Copy + Default + PartialEq + std::fmt::Debug
{
    default fn is_proper_subgraph( &self, subgraph: &Self ) -> bool {
        self.0.data != subgraph.0.data && self.is_subgraph( subgraph )
    }
}

impl<I, N, E> IsImproperSubgraph<I, N, E> for Graph<HashRepr<I, N, E>>
where
    I: Clone + Ord + std::hash::Hash,
    N: Copy + Default + PartialEq + std::fmt::Debug,
    E: Copy + Default + PartialEq + std::fmt::Debug
{
    default fn is_improper_subgraph( &self, subgraph: &Self ) -> bool {
        self.0.data == subgraph.0.data
    }
}

impl<I, N, E> IsSpanningSubgraph<I, N, E> for Graph<HashRepr<I, N, E>>
where
    I: Clone + Ord + std::hash::Hash,
    N: Copy + Default + PartialEq + std::fmt::Debug,
    E: Copy + Default + PartialEq + std::fmt::Debug
{
    default fn is_spanning_subgraph( &self, subgraph: &Self ) -> bool {
        self.0.data.len() == subgraph.data().len() && self.is_subgraph( subgraph )
    }
}

impl<I, N, E> AreAdjacentNodes<I, N, E> for Graph<HashRepr<I, N, E>>
where
    I: Clone + Ord + std::hash::Hash,
    N: Copy + Default + PartialEq + std::fmt::Debug,
    E: Copy + Default + PartialEq + std::fmt::Debug
{
    default fn are_adjacent_nodes( &self, node_1: I, node_2: I ) -> bool {
        self.is_child_node( node_1.clone() )
            && self.is_child_node( node_2.clone() )
            && self.0.data.get( &node_1 ).unwrap().adjacencies().contains_key( &node_2 )
    }
}

impl<I, N, E> AreAdjacentEdges<I, N, E> for Graph<HashRepr<I, N, E>>
where
    I: Clone + Ord + std::hash::Hash,
    N: Copy + Default + PartialEq + std::fmt::Debug,
    E: Copy + Default + PartialEq + std::fmt::Debug
{
    default fn are_adjacent_edges( &self, node_1: I, node_2: I, node_3: I ) -> bool {
        self.are_adjacent_nodes( node_1, node_2.clone() )
            && self.are_adjacent_nodes( node_2, node_3 )
    }
}

impl<I, N, E> Order<I, N, E> for Graph<HashRepr<I, N, E>>
where
    I: Clone + Ord + std::hash::Hash,
    N: Copy + Default + PartialEq + std::fmt::Debug,
    E: Copy + Default + PartialEq + std::fmt::Debug
{
    default fn order( &self ) -> usize {
        self.0.data.len()
    }
}

impl<I, N, E> Size<I, N, E> for Graph<HashRepr<I, N, E>>
where
    I: Clone + Ord + std::hash::Hash,
    N: Copy + Default + PartialEq + std::fmt::Debug,
    E: Copy + Default + PartialEq + std::fmt::Debug
{
    default fn size( &self ) -> usize {
        self.0.data.values().map( |neighbors| neighbors.adjacencies().len() ).sum::<usize>() / 2
    }
}

impl<I, N, E> GetNode<I, N> for Graph<BTreeRepr<I, N, E>>
where
    I: Ord,
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn node( &self, id: I ) -> Option<&N> {
        self.0.data.get( &id ).map( |node| &node.0 )
    }
}

impl<I, N, E> GetNodeMut<I, N> for Graph<BTreeRepr<I, N, E>>
where
    I: Ord,
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn node_mut( &mut self, id: I ) -> Option<&mut N> {
        self.0.data.get_mut( &id ).map( |node| &mut node.0 )
    }
}

impl<I, N, E> GetEdge<I, E> for Graph<BTreeRepr<I, N, E>>
where
    I: Ord,
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn edge( &self, id1: I, id2: I ) -> Option<&E> {
        self.0.data.get( &id1 ).and_then( |node| node.1.get( &id2 ) )
    }
}

impl<I, N, E> GetEdgeMut<I, E> for Graph<BTreeRepr<I, N, E>>
where
    I: Ord,
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn edge_mut( &mut self, id1: I, id2: I ) -> Option<&mut E> {
        self.0.data.get_mut( &id1 ).and_then( |node| node.1.get_mut( &id2 ) )
    }
}

impl<I, N, E> AddNode<I, N> for Graph<BTreeRepr<I, N, E>>
where
    I: Ord,
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn add_node( &mut self, id: I, node: N ) {
        self.0.data.insert( id, ( node, HashMap::default() ) );
    }
}

impl<I, N, E> RemoveNode<I, N> for Graph<BTreeRepr<I, N, E>>
where
    I: Ord,
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn remove_node( &mut self, id: I ) -> Option<N> {
        self.0.data.remove( &id ).map( |node| node.0 )
    }
}

impl<I, N, E> AddEdge<I, E> for Graph<BTreeRepr<I, N, E>>
where
    I: Ord,
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn add_edge( &mut self, id1: I, id2: I, edge: E ) {
        self.0.data.get_mut( &id1 ).map( |node| node.1.insert( id2, edge ) );
    }
}

impl<I, N, E> RemoveEdge<I, E> for Graph<BTreeRepr<I, N, E>>
where
    I: Ord,
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn remove_edge( &mut self, id1: I, id2: I ) -> Option<E> {
        self.0.data.get_mut( &id1 ).and_then( |node| node.1.remove( &id2 ) )
    }
}

impl<I, N, E> ContainsNode<I> for Graph<BTreeRepr<I, N, E>>
where
    I: Ord,
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn contains_node( &self, id: I ) -> bool {
        self.0.data.contains_key( &id )
    }
}

impl<I, N, E> ContainsEdge<I> for Graph<BTreeRepr<I, N, E>>
where
    I: Ord,
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn contains_edge( &self, id1: I, id2: I ) -> bool {
        self.0.data.get( &id1 ).is_some_and( |node| node.1.contains_key( &id2 ) )
    }
}

impl<I, N, E> ClearNodes for Graph<BTreeRepr<I, N, E>>
where
    I: Ord,
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn clear_nodes( &mut self ) {
        self.0.data.clear();
    }
}

impl<I, N, E> ClearEdges for Graph<BTreeRepr<I, N, E>>
where
    I: Ord,
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn clear_edges( &mut self ) {
        self.0.data.iter_mut().for_each( |node| node.1.1.clear() );
    }
}

impl<'a, I, N, E> IterNodes<'a, N> for Graph<BTreeRepr<I, N, E>>
where
    I: Ord,
    N: 'a + Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn iter_nodes( &self ) -> impl Iterator<Item = Option<&N>> {
        self.0.data.iter().map( |node| Some( &node.1.0 ) )
    }
}

impl<'a, I, N, E> IterNodesMut<'a, N> for Graph<BTreeRepr<I, N, E>>
where
    I: Ord,
    N: 'a + Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn iter_nodes_mut( &mut self ) -> impl Iterator<Item = Option<&mut N>> {
        self.0.data.iter_mut().map( |node| Some( &mut node.1.0 ) )
    }
}

impl<'a, I, N, E> IterEdges<'a, I, E> for Graph<BTreeRepr<I, N, E>>
where
    I: Ord,
    N: Clone + Copy + Default + std::fmt::Debug,
    E: 'a + Clone + Copy + Default + std::fmt::Debug
{
    fn iter_edges( &self, id: I ) -> impl Iterator<Item = Option<&E>> {
        self.0.data.get( &id ).map( |node| node.1.iter().map( |edge| Some( edge.1 ) ) ).into_iter().flatten()
    }
}

impl<'a, I, N, E> IterEdgesMut<'a, I, E> for Graph<BTreeRepr<I, N, E>>
where
    I: Ord,
    N: Clone + Copy + Default + std::fmt::Debug,
    E: 'a + Clone + Copy + Default + std::fmt::Debug
{
    fn iter_edges_mut( &mut self, id: I ) -> impl Iterator<Item = Option<&mut E>> {
        self.0.data.get_mut( &id ).map( |node| node.1.iter_mut().map( |edge| Some( edge.1 ) ) ).into_iter().flatten()
    }
}

impl<I, N, E> IsComplete<I, N, E> for Graph<BTreeRepr<I, N, E>>
where
    I: Clone + Ord,
    N: Copy + Default + PartialEq + std::fmt::Debug,
    E: Copy + Default + PartialEq + std::fmt::Debug
{
    fn is_complete( &self ) -> bool {
        for ( node, neighbors ) in self.0.data.iter() {
            if neighbors.1.len() != self.0.data.len() - 1 {
                return false;
            }
            for neighbor in neighbors.1.keys() {
                if !self.node( neighbor ).is_some_and( |n| n.1.contains_key( node ) ) {
                    return false;
                }
            }
        }
        true
    }
}

impl<I, N, E> IsEmpty<I, N, E> for Graph<BTreeRepr<I, N, E>>
where
    I: Clone + Ord,
    N: Copy + Default + PartialEq + std::fmt::Debug,
    E: Copy + Default + PartialEq + std::fmt::Debug
{
    default fn is_empty( &self ) -> bool {
        self.0.data.values().all( |neighbors| neighbors.1.is_empty() )
    }
}

impl<I, N, E> IsTrivial<I, N, E> for Graph<BTreeRepr<I, N, E>>
where
    I: Clone + Ord,
    N: Copy + Default + PartialEq + std::fmt::Debug,
    E: Copy + Default + PartialEq + std::fmt::Debug
{
    default fn is_trivial( &self ) -> bool {
        self.0.data.len() == 1 && self.0.data.values().next().is_some_and( |neighbors| neighbors.1.is_empty())
    }
}

impl<I, N, E> IsNull<I, N, E> for Graph<BTreeRepr<I, N, E>>
where
    I: Clone + Ord,
    N: Copy + Default + PartialEq + std::fmt::Debug,
    E: Copy + Default + PartialEq + std::fmt::Debug
{
    default fn is_null( &self ) -> bool {
        self.0.data.is_empty()
    }
}

impl<I, N, E> IsChildNode<I, N, E> for Graph<BTreeRepr<I, N, E>>
where
    I: Clone + Ord,
    N: Copy + Default + PartialEq + std::fmt::Debug,
    E: Copy + Default + PartialEq + std::fmt::Debug
{
    default fn is_child_node( &self, node_1: I ) -> bool {
        self.contains_node( node_1 )
    }
}

impl<I, N, E> IsSubgraph<I, N, E> for Graph<BTreeRepr<I, N, E>>
where
    I: Clone + Ord,
    N: Copy + Default + PartialEq + std::fmt::Debug,
    E: Copy + Default + PartialEq + std::fmt::Debug
{
    default fn is_subgraph(&self, subgraph: &Self) -> bool {
        subgraph.0.data.iter().all( |(node, neighbors)| {
            self.0.data.get( node ).is_some_and( |graph_node| {
                neighbors.adjacencies().keys().all( |key| graph_node.1.contains_key( key ) )
            })
        })
    }
}

impl<I, N, E> IsProperSubgraph<I, N, E> for Graph<BTreeRepr<I, N, E>>
where
    I: Clone + Ord,
    N: Copy + Default + PartialEq + std::fmt::Debug,
    E: Copy + Default + PartialEq + std::fmt::Debug
{
    default fn is_proper_subgraph( &self, subgraph: &Self ) -> bool {
        self.0.data != subgraph.0.data && self.is_subgraph( subgraph )
    }
}

impl<I, N, E> IsImproperSubgraph<I, N, E> for Graph<BTreeRepr<I, N, E>>
where
    I: Clone + Ord,
    N: Copy + Default + PartialEq + std::fmt::Debug,
    E: Copy + Default + PartialEq + std::fmt::Debug
{
    default fn is_improper_subgraph( &self, subgraph: &Self ) -> bool {
        self.0.data == subgraph.0.data
    }
}

impl<I, N, E> IsSpanningSubgraph<I, N, E> for Graph<BTreeRepr<I, N, E>>
where
    I: Clone + Ord,
    N: Copy + Default + PartialEq + std::fmt::Debug,
    E: Copy + Default + PartialEq + std::fmt::Debug
{
    default fn is_spanning_subgraph( &self, subgraph: &Self ) -> bool {
        self.0.data.len() == subgraph.0.data.len() && self.is_subgraph( subgraph )
    }
}

impl<I, N, E> AreAdjacentNodes<I, N, E> for Graph<BTreeRepr<I, N, E>>
where
    I: Clone + Ord,
    N: Copy + Default + PartialEq + std::fmt::Debug,
    E: Copy + Default + PartialEq + std::fmt::Debug
{
    default fn are_adjacent_nodes( &self, node_1: I, node_2: I ) -> bool {
        self.is_child_node( node_1.clone() )
            && self.is_child_node( node_2.clone() )
            && self.0.data.get( &node_1 ).unwrap().1.contains_key( &node_2 )
    }
}

impl<I, N, E> AreAdjacentEdges<I, N, E> for Graph<BTreeRepr<I, N, E>>
where
    I: Clone + Ord,
    N: Copy + Default + PartialEq + std::fmt::Debug,
    E: Copy + Default + PartialEq + std::fmt::Debug
{
    default fn are_adjacent_edges( &self, node_1: I, node_2: I, node_3: I ) -> bool {
        self.are_adjacent_nodes( node_1, node_2.clone() )
            && self.are_adjacent_nodes( node_2, node_3 )
    }
}

impl<I, N, E> Order<I, N, E> for Graph<BTreeRepr<I, N, E>>
where
    I: Clone + Ord,
    N: Copy + Default + PartialEq + std::fmt::Debug,
    E: Copy + Default + PartialEq + std::fmt::Debug
{
    default fn order( &self ) -> usize {
        self.0.data.len()
    }
}

impl<I, N, E> Size<I, N, E> for Graph<BTreeRepr<I, N, E>>
where
    I: Clone + Ord,
    N: Copy + Default + PartialEq + std::fmt::Debug,
    E: Copy + Default + PartialEq + std::fmt::Debug
{
    default fn size( &self ) -> usize {
        self.0.data.values().map( |neighbors| neighbors.1.len() ).sum::<usize>() / 2
    }
}
