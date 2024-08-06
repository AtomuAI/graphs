// Copyright 2024 Shane W. Mulcahy

#include "intf/graph.hpp"

using namespace atom;

int main() {
    graph::Graph<std::string> graph;

    std::cout << "Is Empty: " << std::boolalpha << graph::is_empty( graph ) << std::endl;

    graph.add_node( "A" );
    graph.add_node( "B" );
    graph.add_node( "C" );
    graph.add_node( "D" );

    std::cout << "Is Empty: " << std::boolalpha << graph::is_empty( graph ) << std::endl;

    graph.add_edge( "A", "B" );
    graph.add_edge( "B", "C" );
    graph.add_edge( "C", "D" );
    graph.add_edge( "D", "A" );
    graph.add_edge( "A", "C" );
    graph.add_edge( "B", "D" );

    std::cout << "Is Empty: " << std::boolalpha << graph::is_empty( graph ) << std::endl;

    std::cout << "Is Complete: " << std::boolalpha << graph::is_complete( graph ) << std::endl;

    graph.remove_edge( "A", "B" );

    std::cout << "Is Complete: " << std::boolalpha << graph::is_complete( graph ) << std::endl;

    std::cout << "Graph: " << std::endl << graph << std::endl;

    graph::generate_dot_graph( graph, "graph.dot" );
}