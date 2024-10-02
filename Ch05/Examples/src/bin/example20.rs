trait Graph {
    type Node;
    type Edge;

    fn nodes(&self) -> Vec<Self::Node>;
    fn edges(&self) -> Vec<Self::Edge>;
}

struct SimpleGraph;

impl Graph for SimpleGraph {
    type Node = i32;
    type Edge = (i32, i32);

    fn nodes(&self) -> Vec<Self::Node> {
        vec![1, 2, 3]
    }

    fn edges(&self) -> Vec<Self::Edge> {
        vec![(1, 2), (2, 3)]
    }
}

fn main() {
    let simple_graph = SimpleGraph;

    println!("{:?}", simple_graph.nodes());
    println!("{:?}", simple_graph.edges());
}
