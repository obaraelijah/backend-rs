use std::collections::VecDeque;

struct Node {
    numbers_of_keys: usize,
    keys: Vec<u32>, // At least t - 1 keys, at most 2t - 1 keys
    children: Vec<Box<Node>>, // At least t children, at most 2t children
    is_leaf: bool,
} 

const MINIMUM_DEGREE: usize = 2; // t
const MAX_DEGREE: usize = 2 * MINIMUM_DEGREE - 1;

impl Node {
    pub fn new(is_leaf: bool) -> Self {
        Node {
            numbers_of_keys: 0,
            keys: vec![],
            children: Vec::new(),
            is_leaf,
        }
    }

    pub fn search(&self, key: &u32) -> Option<&u32> {
       match self.keys.iter().position(|&k| k >= *key) {
            Some(i) if self.keys[i] == *key => Some(&self.keys[i]),
            Some(i) if !self.is_leaf => self.children[i].search(key),
            _ => None,
       }
    }

    pub fn split_child(&mut self, index: usize) {
        let child = match self.children.remove(index) {
            Some(c) => c,
            None => return,
        }

        let mut new_node= Self::new(child.is_leaf);
        new_node.numbers_of_keys = new_node_number_of_keys;

        // Move keys[MINIMUM_DEGREE..] to new node
        for _ in 0..new_node_number_of_keys {
            let key = child.keys.remove(MINIMUM_DEGREE);
            new_node.keys.push(key);
        }

        // Move childrens[MINIMUM_DEGREE] to new node if not leaf node
        if !child.is_leaf {
            for _ in 0..MINIMUM_DEGREE {
                let node = child.children.remove(MINIMUM_DEGREE);
                new_node.children.push(node);
            }
        }

        // x.keys(i) = y.key(t)
        if let Some(key) = child.keys.pop() {
            self.keys.insert(index, key)
        }

        // x.c(i+1) = z
        self.children.insert(index + 1, new_node);

        // x.n = x.n + 1
        self.numbers_of_keys += 1;
    }

    pub fn insert_non_full(&mut self, key: u32) {
        if self.is_leaf {
            match self.keys.binary_search(&key) {
                Ok(_) => (),
                Err(pos) => {
                    self.keys.insert(pos, key);
                }
            }

            self.numbers_of_keys += 1;
        } else {
            let mut index = self.numbers_of_keys - 1;

            while index > 0 && key < self.keys[index] {
                index -= 1;
            }

            if key > self.keys[index] {
                index += 1;
            }

            if self.childrens[index].numbers_of_keys == MAX_DEGREE {
                self.split_child(index);

                if key > self.keys[index] {
                    index += 1
                }
            }

            self.childrens[index].insert_non_full(key)
        }
    }
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Node {{ keys: {:?}, numbers_of_keys: {} }}",
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