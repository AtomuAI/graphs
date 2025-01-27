// Copyright 2024 Bewusstsein Labs

//: Standard
use std::{
    cmp::{ Eq, Ord, PartialEq },
    collections::{ BTreeMap, HashMap },
    marker::PhantomData,
    ops::{ Deref, DerefMut, Not }
};

use crate::{
    graph::{
        Graph,
        Directional,
        Cyclical
    },
    graph_repr::StaticRepr,
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
        IterPair,
        IterPairMut,
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

impl<D, C, N, E, const SIZE: usize> GetNode<usize, N> for Graph<D, C, StaticRepr<N, E, SIZE>>
where
    D: Directional,
    C: Cyclical,
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn node( &self, id: usize ) -> Option<&N> {
        Some( &self.0.0[ id ].node )
    }
}

impl<D, C, N, E, const SIZE: usize> GetNodeMut<usize, N> for Graph<D, C, StaticRepr<N, E, SIZE>>
where
    D: Directional,
    C: Cyclical,
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn node_mut( &mut self, id: usize ) -> Option<&mut N> {
        Some( &mut self.0.0[ id ].node )
    }
}

impl<D, C, N, E, const SIZE: usize> GetEdge<usize, E> for Graph<D, C, StaticRepr<N, E, SIZE>>
where
    D: Directional,
    C: Cyclical,
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn edge( &self, id1: usize, id2: usize ) -> Option<&E> {
        self.0.0[ id1 ].adjs[ id2 ].as_ref()
    }
}

impl<D, C, N, E, const SIZE: usize> GetEdgeMut<usize, E> for Graph<D, C, StaticRepr<N, E, SIZE>>
where
    D: Directional,
    C: Cyclical,
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn edge_mut( &mut self, id1: usize, id2: usize ) -> Option<&mut E> {
        self.0.0[ id1 ].adjs[ id2 ].as_mut()
    }
}

impl<D, C, N, E, const SIZE: usize> ClearEdges for Graph<D, C, StaticRepr<N, E, SIZE>>
where
    D: Directional,
    C: Cyclical,
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn clear_edges( &mut self ) {
        self.0.0.iter_mut().for_each( |pair| pair.adjs.iter_mut().for_each( |edge| *edge = None ));
    }
}

impl<D, C, N, E, const SIZE: usize> IterNodes<N> for Graph<D, C, StaticRepr<N, E, SIZE>>
where
    D: Directional,
    C: Cyclical,
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn iter_nodes( &self ) -> impl Iterator<Item = Option<&N>> {
        self.0.0.iter().map( |pair| Some( &pair.node ) )
    }
}

impl<D, C, N, E, const SIZE: usize> IterNodesMut<N> for Graph<D, C, StaticRepr<N, E, SIZE>>
where
    D: Directional,
    C: Cyclical,
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn iter_nodes_mut( &mut self ) -> impl Iterator<Item = Option<&mut N>> {
        self.0.0.iter_mut().map( |pair| Some( &mut pair.node ) )
    }
}

impl<D, C, N, E, const SIZE: usize> IterEdges<usize, E> for Graph<D, C, StaticRepr<N, E, SIZE>>
where
    D: Directional,
    C: Cyclical,
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn iter_edges( &self, id: usize ) -> impl Iterator<Item = Option<&E>> {
        self.0.0[ id ].adjs.iter().map( |edge| edge.as_ref() )
    }
}

impl<D, C, N, E, const SIZE: usize> IterEdgesMut<usize, E> for Graph<D, C, StaticRepr<N, E, SIZE>>
where
    D: Directional,
    C: Cyclical,
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn iter_edges_mut( &mut self, id: usize ) -> impl Iterator<Item = Option<&mut E>> {
        self.0.0[ id ].adjs.iter_mut().map( |edge| edge.as_mut() )
    }
}

impl<'a, D, C, N, E, const SIZE: usize> IterPair<'a, N, E> for Graph<D, C, StaticRepr<N, E, SIZE>>
where
    D: Directional,
    C: Cyclical,
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn iter_pair( &self ) -> impl Iterator<Item = ( Option<&N>, impl Iterator<Item = Option<&E>> )> {
        self.0.0.iter().map( |node| ( Some( &node.node ), node.adjs.iter().map( |edge| edge.as_ref() ) ) )
    }
}

impl<'a, D, C, N, E, const SIZE: usize> IterPairMut<'a, N, E> for Graph<D, C, StaticRepr<N, E, SIZE>>
where
    D: Directional,
    C: Cyclical,
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn iter_pair_mut( &mut self ) -> impl Iterator<Item = ( Option<&mut N>, impl Iterator<Item = Option<&mut E>> )> {
        self.0.0.iter_mut().map( |pair| ( Some( &mut pair.node ), pair.adjs.iter_mut().map( |edge| edge.as_mut() ) ) )
    }
}

impl<D, C, N, E, const SIZE: usize> IsComplete<usize, N, E> for Graph<D, C, StaticRepr<N, E, SIZE>>
where
    D: Directional,
    C: Cyclical,
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn is_complete( &self ) -> bool {
        self.0.0.iter().any(
            |node| node.adjs.iter().any( |edge| edge.is_none() )
        ).not()
    }
}

impl<D, C, N, E, const SIZE: usize> IsEmpty<usize, N, E> for Graph<D, C, StaticRepr<N, E, SIZE>>
where
    D: Directional,
    C: Cyclical,
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn is_empty( &self ) -> bool {
        self.0.0.iter().any(
            |node| node.adjs.iter().any( |edge| edge.is_some() )
        ).not()
    }
}

impl<D, C, N, E, const SIZE: usize> IsTrivial<usize, N, E> for Graph<D, C, StaticRepr<N, E, SIZE>>
where
    D: Directional,
    C: Cyclical,
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn is_trivial( &self ) -> bool {
        self.0.data.len() == 1 && self.data().values().next().is_some_and( |neighbors| neighbors.adjacencies().is_empty())
    }
}

impl<D, C, N, E, const SIZE: usize> IsNull<usize, N, E> for Graph<D, C, StaticRepr<N, E, SIZE>>
where
    D: Directional,
    C: Cyclical,
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn is_null( &self ) -> bool {
        self.0.data.is_empty()
    }
}

impl<D, C, N, E, const SIZE: usize> IsChildNode<usize, N, E> for Graph<D, C, StaticRepr<N, E, SIZE>>
where
    D: Directional,
    C: Cyclical,
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn is_child_node( &self, node_1: usize ) -> bool {
        self.0.data.contains_node( node_1 )
    }
}

impl<D, C, N, E, const SIZE: usize> IsSubgraph<usize, N, E> for Graph<D, C, StaticRepr<N, E, SIZE>>
where
    D: Directional,
    C: Cyclical,
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

impl<D, C, N, E, const SIZE: usize> IsProperSubgraph<usize, N, E> for Graph<D, C, StaticRepr<N, E, SIZE>>
where
    D: Directional,
    C: Cyclical,
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn is_proper_subgraph( &self, subgraph: &Self ) -> bool {
        self.0.data != subgraph.0.data && self.is_subgraph( subgraph )
    }
}

impl<D, C, N, E, const SIZE: usize> IsImproperSubgraph<usize, N, E> for Graph<D, C, StaticRepr<N, E, SIZE>>
where
    D: Directional,
    C: Cyclical,
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn is_improper_subgraph( &self, subgraph: &Self ) -> bool {
        self.0.data == subgraph.0.data
    }
}

impl<D, C, N, E, const SIZE: usize> IsSpanningSubgraph<usize, N, E> for Graph<D, C, StaticRepr<N, E, SIZE>>
where
    D: Directional,
    C: Cyclical,
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn is_spanning_subgraph( &self, subgraph: &Self ) -> bool {
        self.0.data.len() == subgraph.data().len() && self.is_subgraph( subgraph )
    }
}

impl<D, C, N, E, const SIZE: usize> AreAdjacentNodes<usize, N, E> for Graph<D, C, StaticRepr<N, E, SIZE>>
where
    D: Directional,
    C: Cyclical,
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn are_adjacent_nodes( &self, node_1: usize, node_2: usize ) -> bool {
        self.is_child_node( node_1.clone() )
            && self.is_child_node( node_2.clone() )
            && self.0.data.get( &node_1 ).unwrap().adjacencies().contains_key( &node_2 )
    }
}

impl<D, C, N, E, const SIZE: usize> AreAdjacentEdges<usize, N, E> for Graph<D, C, StaticRepr<N, E, SIZE>>
where
    D: Directional,
    C: Cyclical,
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn are_adjacent_edges( &self, node_1: usize, node_2: usize, node_3: usize ) -> bool {
        self.are_adjacent_nodes( node_1, node_2.clone() )
            && self.are_adjacent_nodes( node_2, node_3 )
    }
}

impl<D, C, N, E, const SIZE: usize> Order<usize, N, E> for Graph<D, C, StaticRepr<N, E, SIZE>>
where
    D: Directional,
    C: Cyclical,
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn order( &self ) -> usize {
        self.0.data.len()
    }
}

impl<D, C, N, E, const SIZE: usize> Size<usize, N, E> for Graph<D, C, StaticRepr<N, E, SIZE>>
where
    D: Directional,
    C: Cyclical,
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    fn size( &self ) -> usize {
        self.0.data.values().map( |neighbors| neighbors.adjacencies().len() ).sum::<usize>() / 2
    }
}
