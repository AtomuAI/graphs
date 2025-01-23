// Copyright 2024 Bewusstsein Labs

#![warn(private_bounds)]

pub mod traverser;

//: Standard
use std::{
    cmp::{ Eq, Ord, PartialEq },
    collections::BTreeMap,
    marker::PhantomData,
    ops::{ Deref, DerefMut }
};

use crate::{
    traits::{
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
    graph_repr::GraphRepr
};

pub trait GraphTraits<I, N, E>
where
    I: Clone + Ord,
    N: PartialEq,
    E: PartialEq
{
    fn add_node( &mut self, id: I, data: N );
    fn get_node( &self, id: I ) -> Option<&N>;
    fn get_node_mut( &mut self, id: I ) -> Option<&mut N>;
    fn contains_node( &self, id: I ) -> bool;
    fn remove_node( &mut self, id: I ) -> Option<N>;
    fn add_edge(&mut self, node1: I, node2: I, data: E);
    fn edge( &self, id1: I, id2: I ) -> Option<&E>;
    fn edge_mut( &mut self, id1: I, id2: I ) -> Option<&mut E>;
    fn contains_edge( &self, id1: I, id2: I ) -> bool;
    fn remove_edge( &mut self, id1: I, id2: I ) -> Option<E>;
    fn clear_edges( &mut self );
}

impl<I, N, E, Repr> GraphTraits<I, N, E> for Graph<Repr>
where
    I: Clone + Ord,
    N: PartialEq,
    E: PartialEq,
    Repr: GraphRepr<I, N, E>
{
    default fn add_node( &mut self, id: I, data: N ) {
        self.data_mut().add_node( id, data )
    }

    default fn get_node( &self, id: I ) -> Option<&N> {
        self.data().node( id )
    }

    default fn get_node_mut( &mut self, id: I ) -> Option<&mut N> {
        self.data_mut().node_mut( id )
    }

    default fn contains_node( &self, id: I ) -> bool {
        self.data().contains_node( id )
    }

    default fn remove_node( &mut self, id: I ) -> Option<N> {
        self.data_mut().remove_node( id )
    }

    default fn add_edge( &mut self, node1: I, node2: I, data: E ) {
        self.data_mut().add_edge( node1, node2, data )
    }

    default fn edge( &self, id1: I, id2: I ) -> Option<&E> {
        self.data().edge( id1, id2 )
    }

    default fn edge_mut( &mut self, id1: I, id2: I ) -> Option<&mut E> {
        self.data_mut().edge_mut( id1, id2 )
    }

    default fn contains_edge( &self, id1: I, id2: I ) -> bool {
        self.data().contains_edge( id1, id2 )
    }

    default fn remove_edge( &mut self, id1: I, id2: I ) -> Option<E> {
        self.data_mut().remove_edge( id1, id2 )
    }

    default fn clear_edges( &mut self ) {
        self.data_mut().clear_edges();
    }
}

impl<I, N, E, Repr> IsComplete<I, N, E> for Graph<Repr>
where
    I: Clone + Ord,
    N: PartialEq,
    E: PartialEq,
    Repr: GraphRepr<I, N, E>
{
    fn is_complete( &self ) -> bool {
        for ( node, neighbors ) in self.data().iter() {
            if neighbors.adjacencies().len() != self.data().len() - 1 {
                return false;
            }
            for neighbor in neighbors.adjacencies().keys() {
                if !self.data().node( neighbor ).is_some_and( |n| n.adjacencies().contains_key( node ) ) {
                    return false;
                }
            }
        }
        true
    }
}

impl<I, N, E, Repr> IsEmpty<I, N, E> for Graph<Repr>
where
    I: Clone + Ord,
    N: PartialEq,
    E: PartialEq,
    Repr: GraphRepr<I, N, E>
{
    default fn is_empty( &self ) -> bool {
        self.data().values().all( |neighbors| neighbors.adjacencies().is_empty() )
    }
}

impl<I, N, E, Repr> IsTrivial<I, N, E> for Graph<Repr>
where
    I: Clone + Ord,
    N: PartialEq,
    E: PartialEq,
    Repr: GraphRepr<I, N, E>
{
    default fn is_trivial( &self ) -> bool {
        self.data().len() == 1 && self.data().values().next().is_some_and( |neighbors| neighbors.adjacencies().is_empty())
    }
}

impl<I, N, E, Repr> IsNull<I, N, E> for Graph<Repr>
where
    I: Clone + Ord,
    N: PartialEq,
    E: PartialEq,
    Repr: GraphRepr<I, N, E>
{
    default fn is_null( &self ) -> bool {
        self.data().is_empty()
    }
}

impl<I, N, E, Repr> IsChildNode<I, N, E> for Graph<Repr>
where
    I: Clone + Ord,
    N: PartialEq,
    E: PartialEq,
    Repr: GraphRepr<I, N, E>
{
    default fn is_child_node( &self, node_1: I ) -> bool {
        self.data().contains_node( node_1 )
    }
}

impl<I, N, E, Repr> IsSubgraph<I, N, E> for Graph<Repr>
where
    I: Clone + Ord,
    N: PartialEq,
    E: PartialEq,
    Repr: GraphRepr<I, N, E>
{
    default fn is_subgraph(&self, subgraph: &'a Self) -> bool {
        subgraph.data().iter().all( |(node, neighbors)| {
            self.data().get( node ).is_some_and( |graph_node| {
                neighbors.adjacencies().keys().all( |key| graph_node.adjacencies().contains_key( key ) )
            })
        })
    }
}

impl<I, N, E, Repr> IsProperSubgraph<I, N, E> for Graph<Repr>
where
    I: Clone + Ord,
    N: PartialEq,
    E: PartialEq,
    Repr: GraphRepr<I, N, E>
{
    default fn is_proper_subgraph( &self, subgraph: &'a Self ) -> bool {
        self.data() != subgraph.data() && self.is_subgraph( subgraph )
    }
}

impl<I, N, E, Repr> IsImproperSubgraph<I, N, E> for Graph<Repr>
where
    I: Clone + Ord,
    N: PartialEq,
    E: PartialEq,
    Repr: GraphRepr<I, N, E>
{
    default fn is_improper_subgraph( &self, subgraph: &'a Self ) -> bool {
        self.data() == subgraph.data()
    }
}

impl<I, N, E, Repr> IsSpanningSubgraph<I, N, E> for Graph<Repr>
where
    I: Clone + Ord,
    N: PartialEq,
    E: PartialEq,
    Repr: GraphRepr<I, N, E>
{
    default fn is_spanning_subgraph( &self, subgraph: &'a Self ) -> bool {
        self.data().len() == subgraph.data().len() && self.is_subgraph( subgraph )
    }
}

impl<I, N, E, Repr> AreAdjacentNodes<I, N, E> for Graph<Repr>
where
    I: Clone + Ord,
    N: PartialEq,
    E: PartialEq,
    Repr: GraphRepr<I, N, E>
{
    default fn are_adjacent_nodes( &self, node_1: I, node_2: I ) -> bool {
        self.is_child_node( node_1.clone() )
            && self.is_child_node( node_2.clone() )
            && self.data().get( &node_1 ).unwrap().adjacencies().contains_key( &node_2 )
    }
}

impl<I, N, E, Repr> AreAdjacentEdges<I, N, E> for Graph<Repr>
where
    I: Clone + Ord,
    N: PartialEq,
    E: PartialEq,
    Repr: GraphRepr<I, N, E>
{
    default fn are_adjacent_edges( &self, node_1: I, node_2: I, node_3: I ) -> bool {
        self.are_adjacent_nodes( node_1, node_2.clone() )
            && self.are_adjacent_nodes( node_2, node_3 )
    }
}

impl<I, N, E, Repr> Order<I, N, E> for Graph<Repr>
where
    I: Clone + Ord,
    N: PartialEq,
    E: PartialEq,
    Repr: GraphRepr<I, N, E>
{
    default fn order( &self ) -> usize {
        self.data().len()
    }
}

impl<I, N, E, Repr> Size<I, N, E> for Graph<Repr>
where
    I: Clone + Ord,
    N: PartialEq,
    E: PartialEq,
    Repr: GraphRepr<I, N, E>
{
    default fn size( &self ) -> usize {
        self.data().values().map( |neighbors| neighbors.adjacencies().len() ).sum::<usize>() / 2
    }
}

pub trait GraphType {}

#[derive( Debug, Clone, Default, PartialEq, Eq )]
pub struct Graph<I, N, E, Repr>( Repr, PhantomData<( I, N, E )> )
where
    I: Clone + Ord,
    N: PartialEq,
    E: PartialEq,
    Repr: GraphRepr<I, N, E>;

impl<I, N, E, Repr> Graph<I, N, E, Repr>
where
    I: Clone + Ord,
    N: PartialEq,
    E: PartialEq,
    Repr: GraphRepr<I, N, E>
{
    pub fn new() -> Self {
        Self ( Repr::default(), PhantomData )
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let graph = Graph::new();
    }
}
