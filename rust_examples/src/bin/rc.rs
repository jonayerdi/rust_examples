use std::rc::Rc;

struct Node {
    id: u32,
    connections: Vec<Rc<Node>>,
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("Dropping Node {}", self.id);
    }
}

fn main() {
    let a = Rc::new(Node {
        id: 1,
        connections: vec![],
    });
    let b = Rc::new(Node {
        id: 2,
        connections: vec![a.clone()],
    });
    let c = Rc::new(Node {
        id: 3,
        connections: vec![a.clone(), b.clone()],
    });
    let d = Rc::new(Node {
        id: 4,
        connections: vec![a.clone(), b.clone(), c.clone()],
    });
    for node in [a, b, c, d].iter() {
        println!("Node {}: {} references", node.id, Rc::strong_count(node));
        for conn in node.connections.iter() {
            println!("{} -> {}", node.id, conn.id);
        }
    }
}
