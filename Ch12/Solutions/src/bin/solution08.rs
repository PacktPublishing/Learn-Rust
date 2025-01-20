use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct GraphNode {
    value: String,
    edges: Vec<Rc<RefCell<GraphNode>>>,
}

impl GraphNode {
    fn new(value: &str) -> Rc<RefCell<GraphNode>> {
        Rc::new(RefCell::new(GraphNode {
            value: value.to_string(),
            edges: Vec::new(),
        }))
    }

    fn add_edge(&mut self, target: Rc<RefCell<GraphNode>>) {
        self.edges.push(target);
    }
}

struct Graph {
    nodes: Vec<Rc<RefCell<GraphNode>>>,
}

impl Graph {
    fn new() -> Self {
        Graph { nodes: Vec::new() }
    }

    fn add_node(&mut self, value: &str) -> Rc<RefCell<GraphNode>> {
        let node = GraphNode::new(value);
        self.nodes.push(Rc::clone(&node));
        node
    }

    fn display(&self) {
        for node in &self.nodes {
            let node_borrow = node.borrow();
            print!("Node '{}': ", node_borrow.value);
            for edge in &node_borrow.edges {
                print!("-> '{}', ", edge.borrow().value);
            }
            println!();
        }
    }
}

fn main() {
    let mut graph = Graph::new();

    let node_a = graph.add_node("A");
    let node_b = graph.add_node("B");
    let node_c = graph.add_node("C");
    let node_d = graph.add_node("D");

    node_a.borrow_mut().add_edge(Rc::clone(&node_b));
    node_a.borrow_mut().add_edge(Rc::clone(&node_c));
    node_b.borrow_mut().add_edge(Rc::clone(&node_d));
    node_c.borrow_mut().add_edge(Rc::clone(&node_d));

    graph.display();
}
