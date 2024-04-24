
pub struct BPlusTree {
    root: Option<Node>,
    max_degree: usize,
}

struct Node {
    keys: Vec<u32>,       // At least t - 1 keys, at most 2t - 1 keys
    values: Vec<u32>,     // Only in leaf node.
    childrens: Vec<Node>, // At least t children, at most 2t children
    is_leaf: bool,
}

impl Node {
    pub fn new(is_leaf: bool) -> Self {
        Node {
            keys: vec![],
            values: vec![],
            childrens: Vec::new(),
            is_leaf,
        }
    }
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Node {{ is_leaf: {}, keys: {:?}, values: {:?}}}",
            self.is_leaf, self.keys, self.values
        )
    }
}

impl BPlusTree {
    pub fn new(numbers: Vec<u32>, max_degree: usize) -> Self {
        let mut tree = Self {
            root: None,
            max_degree
        };

        tree
    }
}

