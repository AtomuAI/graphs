// Copyright 2024 Bewusstsein Labs

pub trait IsComplete<I, N, E> {
    fn is_complete( &self ) -> bool;
}

pub trait IsEmpty<I, N, E> {
    fn is_empty( &self ) -> bool;
}

pub trait IsTrivial<I, N, E> {
    fn is_trivial( &self ) -> bool;
}

pub trait IsNull<I, N, E> {
    fn is_null( &self ) -> bool;
}

pub trait IsChildNode<I, N, E> {
    fn is_child_node( &self, node_1: I ) -> bool;
}

pub trait IsSubgraph<I, N, E> {
    fn is_subgraph( &self, subgraph: &Self ) -> bool;
}

pub trait IsProperSubgraph<I, N, E> {
    fn is_proper_subgraph( &self, subgraph: &Self ) -> bool;
}

pub trait IsImproperSubgraph<I, N, E> {
    fn is_improper_subgraph( &self, subgraph: &Self ) -> bool;
}

pub trait IsSpanningSubgraph<I, N, E> {
    fn is_spanning_subgraph( &self, subgraph: &Self ) -> bool;
}

pub trait AreAdjacentNodes<I, N, E> {
    fn are_adjacent_nodes( &self, node_1: I, node_2: I ) -> bool;
}

pub trait AreAdjacentEdges<I, N, E> {
    fn are_adjacent_edges( &self, node_1: I, node_2: I, node_3: I ) -> bool;
}

pub trait Order<I, N, E> {
    fn order( &self ) -> usize;
}

pub trait Size<I, N, E> {
    fn size( &self ) -> usize;
}
