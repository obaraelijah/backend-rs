
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

    pub fn insert_non_full(&mut self, key: u32, max_degree: usize) {
        match self.keys.binary_search(&key) {
            // Ignore if key is duplicated first
            Ok(_index) => (),
            Err(index) => {
                if self.is_leaf {
                    self.keys.insert(index, key);
                    self.values.insert(index, key);
                } else {
                    self.childrens[index].insert_non_full(key, max_degree);

                    if self.childrens[index].keys.len() == max_degree {
                        self.split_child(index, max_degree);
                    }
                }
            }
        }
    }

    pub fn split_child(&mut self, index: usize, max_degree: usize) {
        if let Some(child) = self.childrens.get_mut(index) {
            let mut right_node = Node::new(child.is_leaf);
            let breakpoint = max_degree / 2;
            let min_number_of_keys = child.keys.len() - breakpoint;

            // TODO: We probably want to rewrite the following parts
            // in a more concise a clear way.
            if index > self.keys.len() {
                if self.is_leaf {
                    self.values.push(child.values[breakpoint]);
                }
                self.keys.push(child.keys[breakpoint]);
            } else {
                // TODO: Add explanation why this is needed
                if self.is_leaf {
                    self.values.insert(index, child.values[breakpoint]);
                }
                self.keys.insert(index, child.keys[breakpoint]);
            }

            for i in 0..min_number_of_keys {
                let key = child.keys.remove(breakpoint);

                // If is leaf child, we split all the keys to the right node,
                // including the key we move to parent.
                //
                // If is internal node, there's no need to the breakpoint key
                // to the right node as it is available at the child level.
                //
                // Hence, we will skip the first key when it is not a leaf node.
                if child.is_leaf || i != 0 {
                    right_node.keys.push(key);
                }

                if child.is_leaf {
                    let value = child.values.remove(breakpoint);
                    right_node.values.push(value);
                }
            }

            // If node is leaf, means there's not children
            if !child.is_leaf {
                for _ in 0..min_number_of_keys {
                    let value = child.childrens.remove(breakpoint + 1);
                    right_node.childrens.push(value);
                }

                // Since we now have childrens, we are not leaf node anymore.
                right_node.is_leaf = false;
                right_node.values.clear();
            }

            // TODO: Add explanation why this is needed
            if index + 1 > self.childrens.len() {
                self.childrens.push(right_node);
            } else {
                self.childrens.insert(index + 1, right_node);
            }
        }
    }

    pub fn search(&self, key: &u32) -> Option<&u32> {
        match self.keys.binary_search(key) {
            Ok(index) => {
                if self.is_leaf {
                    self.values.get(index)
                } else {
                    self.childrens[index + 1].search(key)
                }
            }
            Err(index) => {
                if self.is_leaf {
                    None
                } else {
                    self.childrens[index].search(key)
                }
            }
        }
    }

    fn min_key(&self, max_degree: usize) -> usize {
        let mut min_key = (max_degree / 2) - 1;

        if min_key == 0 {
            min_key = 1;
        }

        min_key
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

