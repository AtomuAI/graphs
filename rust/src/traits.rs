// Copyright 2024 Bewusstsein Labs

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
    fn remove_node( &mut self, id: I ) -> N;
}

pub trait AddEdge<I, E> {
    fn add_edge( &mut self, id1: I, id2: I, edge: E );
}

pub trait RemoveEdge<I, E> {
    fn remove_edge( &mut self, id1: I, id2: I ) -> E;
}

pub trait ContainsNode<I, N> {
    fn contains_node( &self, id: I ) -> bool;
}

pub trait ContainsEdge<I, E> {
    fn contains_edge( &self, id1: I, id2: I ) -> bool;
}

pub trait ClearNodes {
    fn clear_nodes( &mut self );
}

pub trait ClearEdges {
    fn clear_edges( &mut self );
}

pub trait IterNodes<N>
{
    fn iter_nodes( &self ) -> impl Iterator<Item = Option<&N>>;
}

pub trait IterNodesMut<N>
{
    fn iter_nodes_mut( &mut self ) -> impl Iterator<Item = Option<&mut N>>;
}

pub trait IterEdges<I, E>
{
    fn iter_edges( &self, id: I ) -> impl Iterator<Item = Option<&E>>;
}

pub trait IterEdgesMut<I, E>
{
    fn iter_edges_mut( &mut self, id: I ) -> impl Iterator<Item = Option<&mut E>>;
}

pub trait IterPair<'a, N, E>
where
    N: 'a,
    E: 'a
{
    fn iter_pair( &'a self ) -> impl Iterator<Item = ( Option<&'a N>, impl Iterator<Item = Option<&'a E>> )>;
}

pub trait IterPairMut<'a, N, E>
where
    N: 'a,
    E: 'a
{
    fn iter_pair_mut( &'a mut self ) -> impl Iterator<Item = ( Option<&'a mut N>, impl Iterator<Item = Option<&'a mut E>> )>;
}

pub trait IsComplete<I, N, E> {
    fn is_complete( &self ) -> bool;
}

pub trait IsEmpty<N, E> {
    fn is_empty( &self ) -> bool;
}

pub trait IsTrivial<N, E> {
    fn is_trivial( &self ) -> bool;
}

pub trait IsNull<N, E> {
    fn is_null( &self ) -> bool;
}

pub trait IsChildNode<N, E> {
    fn is_child_node( &self, node_1: I ) -> bool;
}

pub trait IsSubgraph<N, E> {
    fn is_subgraph( &self, subgraph: &Self ) -> bool;
}

pub trait IsProperSubgraph<N, E> {
    fn is_proper_subgraph( &self, subgraph: &Self ) -> bool;
}

pub trait IsImproperSubgraph<N, E> {
    fn is_improper_subgraph( &self, subgraph: &Self ) -> bool;
}

pub trait IsSpanningSubgraph<N, E> {
    fn is_spanning_subgraph( &self, subgraph: &Self ) -> bool;
}

pub trait AreAdjacentNodes<N, E> {
    fn are_adjacent_nodes( &self, node_1: I, node_2: I ) -> bool;
}

pub trait AreAdjacentEdges<N, E> {
    fn are_adjacent_edges( &self, node_1: I, node_2: I, node_3: I ) -> bool;
}

pub trait Order<N, E> {
    fn order( &self ) -> usize;
}

pub trait Size<N, E> {
    fn size( &self ) -> usize;
}
