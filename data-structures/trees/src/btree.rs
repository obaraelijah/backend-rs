use std::collections::VecDeque;

pub struct BTree {
    root: Option<Box<Node>>,
}

struct Node {
    numbers_of_keys: usize,    // 2t ^ h - 1.
    keys: Vec<u32>,            // At least t - 1 keys, at most 2t - 1 keys
    childrens: Vec<Box<Node>>, // At least t children, at most 2t children
    is_leaf: bool,
}

const MINIMUM_DEGREE: usize = 2; // t
const MAX_DEGREE: usize = 2 * MINIMUM_DEGREE - 1;

impl Node {
    pub fn new(is_leaf: bool) -> Self {
        Node {
            numbers_of_keys: 0,
            keys: vec![],
            childrens: Vec::new(),
            is_leaf,
        }
    }

    pub fn search(&self, key: &u32) -> Option<&u32> {
        let mut index = 0;
        let mut node_key = self.keys[index];

        while *key > node_key {
            index += 1;

            if index >= self.numbers_of_keys {
                break;
            }

            node_key = self.keys[index];
        }

        if index < self.numbers_of_keys && *key == node_key {
            return self.keys.get(index);
        } else if self.is_leaf {
            return None;
        } else {
            let next_node = &self.childrens[index];
            return next_node.search(key);
        }
    }

    pub fn split_child(&mut self, index: usize) {
        if let Some(child) = self.childrens.get_mut(index) {
            let mut new_node = Self::new(child.is_leaf);
            let new_node_number_of_keys = MINIMUM_DEGREE - 1;
            new_node.numbers_of_keys = new_node_number_of_keys;

            // Move keys[t..] to new node
            // for j = 1 to t - 1
            //   z.key(k) = y.key(j + t)
            // y.n = t - 1
            for j in 0..new_node_number_of_keys {
                let key = child.keys.remove(MINIMUM_DEGREE);
                new_node.keys.insert(j, key);
                child.numbers_of_keys -= 1;
            }

            // Move childrens[t..] to new node if not leaf node
            // if not y.leaf
            //   for j = 1 to t
            //     z.c(j) = y.c(j+t)
            if !child.is_leaf {
                for j in 0..MINIMUM_DEGREE {
                    let nodes = child.childrens.remove(MINIMUM_DEGREE);
                    new_node.childrens.insert(j, nodes);
                }
            }

            // x.key(i) = y.key(t)
            if let Some(key) = child.keys.pop() {
                self.keys.insert(index, key);
                child.numbers_of_keys -= 1;
            }

            // x.c(i+1) = z
            self.childrens.insert(index + 1, Box::new(new_node));

            // x.n = x.n + 1
            self.numbers_of_keys += 1;
        };
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

    // Merge both child
    //     4  |  7
    //    /   |   \
    //   1    6  8|9
    //
    // Delete 4:
    //
    //         7
    //     /      \
    //   1|4|6    8|9
    //
    //        7
    //      /   \
    //    1|6   8|9

    // Get left and right child

    // if key = 18
    //     16 | 18 | 20
    //    /   /    \   \
    //   14  17    19  21
    //
    // left = 17
    // right = 19
    pub fn merge_childs(&mut self, index: usize) {
        let key = self.keys.remove(index);
        self.numbers_of_keys -= 1;

        let left = self.childrens.remove(index);
        let mut right = self.childrens.remove(index);

        println!("Merging {:?}, {key}, {:?}...", left.keys, right.keys);

        // Merge the keys
        let mut new_keys = left.keys;
        new_keys.push(key);
        new_keys.append(&mut right.keys);

        let mut left_chidrens = left.childrens;
        let mut right_childrens = right.childrens;
        left_chidrens.append(&mut right_childrens);

        let node = Node {
            numbers_of_keys: left.numbers_of_keys + right.numbers_of_keys + 1,
            is_leaf: left.is_leaf,
            keys: new_keys,
            childrens: left_chidrens,
        };

        self.childrens.insert(index, Box::new(node));
    }

    pub fn remove_from_internals(&mut self, index: usize) -> Option<u32> {
        let key = self.keys[index];

        if self.childrens[index].numbers_of_keys >= MINIMUM_DEGREE {
            println!("Swap with left child...");
            //     4  |  7
            //    /   |   \
            //   1|2  6  8|9
            //
            // Delete 4:
            //
            //     2  |  7
            //    /   |   \
            //   1    6   8|9

            // Recursively find the biggest left children to be swap:
            let mut most_left = &mut self.childrens[index];

            while let Some(node) = most_left.childrens.last_mut() {
                most_left = node;
            }

            let k1 = most_left.keys[most_left.keys.len() - 1];

            // Swap k1 with key:
            println!("Replace {key} with {k1}: {:?}", self.keys);
            let key = self.keys.remove(index);
            self.keys.insert(index, k1);

            println!("Removing {k1} from {:?}...", self.childrens[index]);
            self.childrens[index].remove(&k1);

            Some(key)
        } else if self.childrens[index + 1].numbers_of_keys >= MINIMUM_DEGREE {
            println!("Swap with right child...");
            //     4  |  7
            //    /   |   \
            //   1   5|6  8|9
            //
            // Delete 4:
            //
            //     5  |  7
            //    /   |   \
            //   1    6  8|9
            let mut most_right = &mut self.childrens[index + 1];

            while let Some(node) = most_right.childrens.first_mut() {
                most_right = node;
            }

            let k1 = most_right.keys[0];

            // Swap k1 with key:
            println!("Replace {key} with {k1}: {:?}", self.keys);
            let key = self.keys.remove(index);
            self.keys.insert(index, k1);

            println!("Removing {k1} from {:?}...", self.childrens[index + 1]);
            self.childrens[index + 1].remove(&k1);

            Some(key)
        } else {
            self.merge_childs(index);
            // Recursively call remove
            self.childrens[index].remove(&key)
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

impl BTree {
    pub fn new() -> BTree {
        BTree { root: None }
    }

    pub fn insert(&mut self, key: u32) {
        if let Some(node) = &mut self.root {
            if node.numbers_of_keys == MAX_DEGREE {
                let mut new_root = Node::new(false);
                new_root.childrens.push(self.root.take().unwrap());
                new_root.split_child(0);
                new_root.insert_non_full(key);
                self.root = Some(Box::new(new_root));
            } else {
                node.insert_non_full(key);
            }
        } else {
            let mut node = Node::new(true);
            node.insert_non_full(key);
            self.root = Some(Box::new(node));
        }
    }
}
