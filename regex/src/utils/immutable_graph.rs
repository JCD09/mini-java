use std::collections::{HashMap, HashSet};

const INITIAL_CAPACITY: usize = 5; 
const MULTIPLIER: usize = 2;


// This is simple graph backed up by Vector of nodes 
// somewhat akin to adjacency list 
pub struct Graph<T> {
    nodes: Vec<T>,
    edges: HashMap<NodeId, HashSet<NodeId>>
}

type NodeId = usize;

impl<T> Graph<T> {
    pub fn new() -> Self {
        Graph {
            nodes: Vec::with_capacity(INITIAL_CAPACITY),
            edges: HashMap::with_capacity(INITIAL_CAPACITY)
            
        }
    }

    pub fn length(&self) -> usize {
        self.length()
    }

    pub fn add_node(&mut self, data: T) {
        if self.is_full() {
            self.extend();
        }

        self.nodes.push(data);
    }

    // inserts edge only when two nodes are defined are defined. 
    pub fn add_edge(&mut self, from: NodeId, to: NodeId) {
        match (self.nodes.get(from), self.nodes.get(to)) {
            (Some(_),Some(_)) => {
                self.edges
                    .entry(from)
                    .or_insert(HashSet::with_capacity(INITIAL_CAPACITY))
                    .insert(to);
            },
            _ => {}
        }
    }

    pub fn get(&mut self, index: NodeId) -> Option<&T> {
        self.nodes.get(index)
    }

    pub fn get_mut(&mut self, index: NodeId) -> Option<&mut T> {
        self.nodes.get_mut(index)
    }

    fn is_full(&self) -> bool {
        self.nodes.len() >= self.nodes.capacity() 
    }

    fn extend(&mut self) {
        let new_capacity = MULTIPLIER*self.nodes.capacity();
        self.nodes.reserve_exact(new_capacity);
    }

}