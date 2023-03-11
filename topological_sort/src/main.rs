use std::{collections::HashMap, rc::Rc, sync::Mutex};

fn main() {
    let mut graph = Graph::new();
    graph.add_vertex(0, vec![]);
    graph.add_vertex(1, vec![]);
    graph.add_vertex(2, vec![3]);
    graph.add_vertex(3, vec![1]);
    graph.add_vertex(4, vec![0, 1]);
    graph.add_vertex(5, vec![0, 2]);

    let sorted = topological_sort(graph);
    println!("{:?}", sorted)
}

fn topological_sort(graph: Graph) -> Vec<u8> {
    let mut sorted: Vec<u8> = vec![];
    for (id, vertex) in graph.vertices {
        if !sorted.contains(&id) {
            println!("\nrandomly visiting {}", id);
            _topological_sort(Option::None, &id, Rc::clone(&vertex), &mut sorted);
        }
    }

    println!("\n sorting finished");
    sorted
}

fn _topological_sort(
    previous: Option<&u8>,
    id: &u8,
    vertex: Rc<Mutex<Vertex>>,
    sorted: &mut Vec<u8>,
) {
    // println!("at {}", id);
    for (connection_id, connection) in &vertex.lock().unwrap().connections {
        if !sorted.contains(&connection_id) {
            println!("traveling to {} from {}", connection_id, id);
            _topological_sort(
                Option::Some(id),
                connection_id,
                Rc::clone(&connection),
                sorted,
            );
        }
    }

    // println!("at {}: all visited", id);

    match previous {
        Some(prev_id) => println!("returning to {} from {}", prev_id, id),
        None => println!("all visited"),
    }

    sorted.push(id.to_owned());
}

#[derive(Debug)]
struct Vertex {
    // id: u8,
    connections: HashMap<u8, Rc<Mutex<Vertex>>>,
}

#[derive(Debug)]
struct Graph {
    vertices: HashMap<u8, Rc<Mutex<Vertex>>>,
}

impl Graph {
    fn new() -> Self {
        Self {
            vertices: Default::default(),
        }
    }

    fn add_vertex(&mut self, id: u8, connection_ids: Vec<u8>) {
        for connection_id in connection_ids {
            let connection = Rc::clone(self.vertices.entry(connection_id).or_insert(Rc::new(
                Mutex::new(Vertex {
                    // id: connection_id,
                    connections: HashMap::new(),
                }),
            )));
            self.vertices
                .entry(id)
                .or_insert(Rc::new(Mutex::new(Vertex {
                    // id,
                    connections: HashMap::new(),
                })))
                .lock()
                .unwrap()
                .connections
                .insert(connection_id, connection);
        }
    }
}
