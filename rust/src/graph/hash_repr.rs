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
    graph_repr::HashRepr,
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
    fn is_empty( &self ) -> bool {
        self.0.data.values().all( |neighbors| neighbors.adjacencies().is_empty() )
    }
}

impl<I, N, E> IsTrivial<I, N, E> for Graph<HashRepr<I, N, E>>
where
    I: Clone + Ord + std::hash::Hash,
    N: Copy + Default + PartialEq + std::fmt::Debug,
    E: Copy + Default + PartialEq + std::fmt::Debug
{
    fn is_trivial( &self ) -> bool {
        self.0.data.len() == 1 && self.data().values().next().is_some_and( |neighbors| neighbors.adjacencies().is_empty())
    }
}

impl<I, N, E> IsNull<I, N, E> for Graph<HashRepr<I, N, E>>
where
    I: Clone + Ord + std::hash::Hash,
    N: Copy + Default + PartialEq + std::fmt::Debug,
    E: Copy + Default + PartialEq + std::fmt::Debug
{
    fn is_null( &self ) -> bool {
        self.0.data.is_empty()
    }
}

impl<I, N, E> IsChildNode<I, N, E> for Graph<HashRepr<I, N, E>>
where
    I: Clone + Ord + std::hash::Hash,
    N: Copy + Default + PartialEq + std::fmt::Debug,
    E: Copy + Default + PartialEq + std::fmt::Debug
{
    fn is_child_node( &self, node_1: I ) -> bool {
        self.0.data.contains_node( node_1 )
    }
}

impl<I, N, E> IsSubgraph<I, N, E> for Graph<HashRepr<I, N, E>>
where
    I: Clone + Ord + std::hash::Hash,
    N: Copy + Default + PartialEq + std::fmt::Debug,
    E: Copy + Default + PartialEq + std::fmt::Debug
{
    fn is_subgraph(&self, subgraph: &Self) -> bool {
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
    fn is_proper_subgraph( &self, subgraph: &Self ) -> bool {
        self.0.data != subgraph.0.data && self.is_subgraph( subgraph )
    }
}

impl<I, N, E> IsImproperSubgraph<I, N, E> for Graph<HashRepr<I, N, E>>
where
    I: Clone + Ord + std::hash::Hash,
    N: Copy + Default + PartialEq + std::fmt::Debug,
    E: Copy + Default + PartialEq + std::fmt::Debug
{
    fn is_improper_subgraph( &self, subgraph: &Self ) -> bool {
        self.0.data == subgraph.0.data
    }
}

impl<I, N, E> IsSpanningSubgraph<I, N, E> for Graph<HashRepr<I, N, E>>
where
    I: Clone + Ord + std::hash::Hash,
    N: Copy + Default + PartialEq + std::fmt::Debug,
    E: Copy + Default + PartialEq + std::fmt::Debug
{
    fn is_spanning_subgraph( &self, subgraph: &Self ) -> bool {
        self.0.data.len() == subgraph.data().len() && self.is_subgraph( subgraph )
    }
}

impl<I, N, E> AreAdjacentNodes<I, N, E> for Graph<HashRepr<I, N, E>>
where
    I: Clone + Ord + std::hash::Hash,
    N: Copy + Default + PartialEq + std::fmt::Debug,
    E: Copy + Default + PartialEq + std::fmt::Debug
{
    fn are_adjacent_nodes( &self, node_1: I, node_2: I ) -> bool {
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
    fn are_adjacent_edges( &self, node_1: I, node_2: I, node_3: I ) -> bool {
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
    fn order( &self ) -> usize {
        self.0.data.len()
    }
}

impl<I, N, E> Size<I, N, E> for Graph<HashRepr<I, N, E>>
where
    I: Clone + Ord + std::hash::Hash,
    N: Copy + Default + PartialEq + std::fmt::Debug,
    E: Copy + Default + PartialEq + std::fmt::Debug
{
    fn size( &self ) -> usize {
        self.0.data.values().map( |neighbors| neighbors.adjacencies().len() ).sum::<usize>() / 2
    }
}
