// Copyright 2024 Bewusstsein Labs

#![warn(private_bounds)]

pub mod static_repr;
pub mod dynamic_repr;
pub mod hash_repr;
pub mod btree_repr;

use std::marker::PhantomData;

use crate::{
    graph_repr::GraphRepr,
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

pub trait Directional {}
pub trait Cyclical {}

pub struct Directed;
pub struct Undirected;
pub struct Cyclic;
pub struct Acyclic;

//pub mod traverser;

pub type GraphType<D, C> = ( D, C );

#[derive( Debug, Clone, Default, PartialEq, Eq )]
pub struct Graph<D, C, R>( R, GraphType<D, C> )
where
    D: Directional,
    C: Cyclical,
    R: GraphRepr;

impl<D, C, I, N, R> ContainsNode<I, N> for Graph<D, C, R>
where
    D: Directional,
    C: Cyclical,
    R: GraphRepr,
    N: Clone + Copy + Default + std::fmt::Debug,
    Self: GetNode<I, N>
{
    default fn contains_node( &self, id: I ) -> bool {
        self.node( id ).is_some()
    }
}

impl<D, C, I, E, R> ContainsEdge<I, E> for Graph<D, C, R>
where
    D: Directional,
    C: Cyclical,
    R: GraphRepr,
    E: Clone + Copy + Default + std::fmt::Debug,
    Self: GetEdge<I, E>
{
    default fn contains_edge( &self, id1: I, id2: I ) -> bool {
        self.edge( id1, id2 ).is_some()
    }
}

impl<'a, D, C, I, N, E, R> IsComplete<I, N, E> for Graph<D, C, R>
where
    D: Directional,
    C: Cyclical,
    R: GraphRepr,
    N: 'a + Clone + Copy + Default + std::fmt::Debug,
    E: 'a + Clone + Copy + Default + std::fmt::Debug,
    Self: IterPair<'a, N, E>
{
    default fn is_complete( &'a self ) -> bool {
        self.iter_pair().all( |( node, mut edges )| {
            edges.all( |edge| edge.is_some() ) && node.is_some()
        })
    }
}

impl<D, C, I, N, E, R> IsEmpty<I, N, E> for Graph<D, C, R>
where
    D: Directional,
    C: Cyclical,
    R: GraphRepr,
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    default fn is_empty( &self ) -> bool {
        //self.0.data.values().all( |neighbors| neighbors.adjacencies().is_empty() )
        self.
    }
}

impl<D, C, I, N, E, R> IsTrivial<I, N, E> for Graph<D, C, R>
where
    D: Directional,
    C: Cyclical,
    R: GraphRepr,
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    default fn is_trivial( &self ) -> bool {
        self.0.data.len() == 1 && self.data().values().next().is_some_and( |neighbors| neighbors.adjacencies().is_empty())
    }
}

impl<D, C, 'a, I, N, E, R> IsNull<I, N, E> for Graph<D, C, R>
where
    D: Directional,
    C: Cyclical,
    R: GraphRepr,
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    default fn is_null( &self ) -> bool {
        self.0.data.is_empty()
    }
}

impl<D, C, I, N, E, R> IsChildNode<I, N, E> for Graph<D, C, R>
where
    D: Directional,
    C: Cyclical,
    R: GraphRepr,
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    default fn is_child_node( &self, node_1: I ) -> bool {
        self.0.data.contains_node( node_1 )
    }
}

impl<D, C, I, N, E, R> IsSubgraph<I, N, E> for Graph<D, C, R>
where
    D: Directional,
    C: Cyclical,
    R: GraphRepr,
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

impl<D, C, I, N, E, R> IsProperSubgraph<I, N, E> for Graph<D, C, R>
where
    D: Directional,
    C: Cyclical,
    R: GraphRepr,
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    default fn is_proper_subgraph( &self, subgraph: &Self ) -> bool {
        self.0.data != subgraph.0.data && self.is_subgraph( subgraph )
    }
}

impl<D, C, I, N, E, R> IsImproperSubgraph<I, N, E> for Graph<D, C, R>
where
    D: Directional,
    C: Cyclical,
    R: GraphRepr,
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    default fn is_improper_subgraph( &self, subgraph: &Self ) -> bool {
        self.0.data == subgraph.0.data
    }
}

impl<D, C, I, N, E, R> IsSpanningSubgraph<I, N, E> for Graph<D, C, R>
where
    D: Directional,
    C: Cyclical,
    R: GraphRepr,
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    default fn is_spanning_subgraph( &self, subgraph: &Self ) -> bool {
        self.0.data.len() == subgraph.data().len() && self.is_subgraph( subgraph )
    }
}

impl<D, C, I, N, E, R> AreAdjacentNodes<I, N, E> for Graph<D, C, R>
where
    D: Directional,
    C: Cyclical,
    R: GraphRepr,
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    default fn are_adjacent_nodes( &self, node_1: I, node_2: I ) -> bool {
        self.is_child_node( node_1.clone() )
            && self.is_child_node( node_2.clone() )
            && self.0.data.get( &node_1 ).unwrap().adjacencies().contains_key( &node_2 )
    }
}

impl<D, C, I, N, E, R> AreAdjacentEdges<I, N, E> for Graph<D, C, R>
where
    D: Directional,
    C: Cyclical,
    R: GraphRepr,
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    default fn are_adjacent_edges( &self, node_1: I, node_2: I, node_3: I ) -> bool {
        self.are_adjacent_nodes( node_1, node_2.clone() )
            && self.are_adjacent_nodes( node_2, node_3 )
    }
}

impl<D, C, I, N, E, R> Order<I, N, E> for Graph<D, C, R>
where
    D: Directional,
    C: Cyclical,
    R: GraphRepr,
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    default fn order( &self ) -> I {
        self.0.data.len()
    }
}

impl<D, C, I, N, E, R> Size<I, N, E> for Graph<D, C, R>
where
    D: Directional,
    C: Cyclical,
    R: GraphRepr,
    N: Clone + Copy + Default + std::fmt::Debug,
    E: Clone + Copy + Default + std::fmt::Debug
{
    default fn size( &self ) -> I {
        self.0.data.values().map( |neighbors| neighbors.adjacencies().len() ).sum::<I>() / 2
    }
}
