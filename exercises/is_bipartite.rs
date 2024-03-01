/*

    let graph: DirectedCsrGraph<usize> = GraphBuilder::new()
      .csr_layout(CsrLayout::Sorted)
      .edges(vec![(0, 1), (0, 2), (1, 2), (1, 3), (2, 3)])
      .build();

*/

use graph::prelude::*;

struct GraphNode {
    node : usize, 
    color: usize
}

pub fn bipartite_graph (graph : &DirectedCsrGraph<usize>, source: usize) -> bool { 
    let mut stack :Vec<GraphNode> = vec![];
    let mut visited: HashMap<usize, usize> = HashMap::new();

    stack.push(GraphNode { node: source, color: 0 });

    while !stack.is_empty() {
        let n = stack.pop().unwrap();

        if visited.contains_key(&n.node) {
            if *visited.get(&n.node).unwrap() != n.color {
                return false;
            }
            continue;
        }

        visited.insert(n.node, n.color);

        for neighbour in graph.out_neighbors(n.node).as_slice() {
            stack.push(GraphNode {
                node : *neighbour, 
                color: 1-n.color
            })
        }
    }

    true
}
