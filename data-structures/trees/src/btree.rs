use std::collections::VecDeque;

struct Node {
    number_of_keys: usize,
    keys: Vec<u32>, // At least t - 1 keys, at most 2t - 1 keys
    children: Vec<Box<Node>>, // At least t children, at most 2t children
    is_leaf: bool,
} 

impl Node {
    pub fn new(is_leaf: bool) -> Self {
        Node {
            number_of_keys: 0,
            keys: vec![],
            children: Vec::new(),
            is_leaf,
        }
    }
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Node {{ keys: {:?}, number_of_keys: {} }}",
            self.keys, self.numbers_of_keys
        )
    }
}

struct BTree {
    root: Option<Box<Node>>,
}

impl BTree {
    pub fn new() -> Self {
        BTree { root: None }
    }
}