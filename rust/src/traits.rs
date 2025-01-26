// Copyright 2024 Bewusstsein Labs

pub trait GetNode<I, N> {
    fn node( &self, id: &I ) -> Option<&N>;
}

pub trait GetNodeMut<I, N> {
    fn node_mut( &mut self, id: &I ) -> Option<&mut N>;
}

pub trait GetEdge<I, E> {
    fn edge( &self, id1: &I, id2: &I ) -> Option<&E>;
}

pub trait GetEdgeMut<I, E> {
    fn edge_mut( &mut self, id1: &I, id2: &I ) -> Option<&mut E>;
}

pub trait AddNode<I, N> {
    fn add_node( &mut self, id: &I, node: N );
}

pub trait RemoveNode<I, N> {
    fn remove_node( &mut self, id: &I ) -> Option<N>;
}

pub trait AddEdge<I, E> {
    fn add_edge( &mut self, id1: &I, id2: &I, edge: E );
}

pub trait RemoveEdge<I, E> {
    fn remove_edge( &mut self, id1: &I, id2: I ) -> Option<E>;
}

pub trait ContainsNode<I> {
    fn contains_node( &self, id: &I ) -> bool;
}

pub trait ContainsEdge<I> {
    fn contains_edge( &self, id1: &I, id2: &I ) -> bool;
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
    fn iter_edges( &'a self, id: &I ) -> impl Iterator<Item = Option<&'a E>>;
}

pub trait IterEdgesMut<'a, I, E>
where
    E: 'a
{
    fn iter_edges_mut( &'a mut self, id: &I ) -> impl Iterator<Item = Option<&'a mut E>>;
}

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
    fn are_adjacent_nodes( &self, node_1: &I, node_2: &I ) -> bool;
}

pub trait AreAdjacentEdges<I, N, E> {
    fn are_adjacent_edges( &self, node_1: &I, node_2: &I, node_3: &I ) -> bool;
}

pub trait Order<I, N, E> {
    fn order( &self ) -> usize;
}

pub trait Size<I, N, E> {
    fn size( &self ) -> usize;
}
