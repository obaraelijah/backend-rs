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

    pub fn fill(&mut self, index: usize) {
        let is_prev = index == (self.childrens.len() - 1);

        // TODO: Add test case and fix this:
        // Currently, we only look at the prev sibling if we are the last
        // child.
        //
        // This is not entirely correct because the actual implementation
        // is to look at both immediate siblings to see if they have t
        // keys. So there will be a case where the fill doesn't happen at all.
        let siblings = if index == self.childrens.len() - 1 {
            self.childrens[index - 1].as_mut()
        } else {
            self.childrens[index + 1].as_mut()
        };

        if siblings.numbers_of_keys >= MINIMUM_DEGREE {
            // Get the last key from our parent
            //
            // It doesn't matter as we will then decide
            // to push to our children end or insert in front
            // depending of whether is a left or right child.
            let k1 = self.keys.pop().unwrap();

            if is_prev {
                let k2 = siblings.keys.pop().unwrap();
                siblings.numbers_of_keys -= 1;
                if !siblings.is_leaf {
                    let child = siblings.childrens.pop().unwrap();
                    self.childrens[index].childrens.insert(0, child);
                }

                println!("Stealing {k2} last value from prev siblings and moving {k1} below as first value...");
                self.childrens[index].keys.insert(0, k1);
                self.childrens[index].numbers_of_keys += 1;
                self.keys.push(k2);
            } else {
                let k2 = siblings.keys.remove(0);
                siblings.numbers_of_keys -= 1;
                if !siblings.is_leaf {
                    let child = siblings.childrens.remove(0);
                    self.childrens[index].childrens.push(child);
                }

                println!("Stealing {k2}, first value from next siblings and insert {k1} below as last value...");
                self.childrens[index].keys.push(k1);
                self.childrens[index].numbers_of_keys += 1;
                self.keys.push(k2);
            }
        } else {
            if index == self.childrens.len() - 1 {
                self.merge_childs(index - 1);
            } else {
                self.merge_childs(index);
            }
        }
    }

    pub fn remove(&mut self, key: &u32) -> Option<u32> {
        println!("--- Remove {key} from {:?}", self.keys);
        match self.keys.binary_search(key) {
            Ok(index) => {
                if self.is_leaf {
                    let key = self.keys.remove(index);
                    self.numbers_of_keys -= 1;
                    Some(key)
                } else {
                    self.remove_from_internals(index)
                }
            }
            Err(index) => {
                println!("Didn't found key in current node, child index: {index}...");
                if self.is_leaf {
                    None
                } else {
                    println!("my children: {:?}", self.childrens);
                    if self.childrens[index].numbers_of_keys == MINIMUM_DEGREE - 1 {
                        println!("Child has less than t keys, fill it up...");
                        self.fill(index);
                    }

                    if index < self.childrens.len() {
                        self.childrens[index].remove(key)
                    } else {
                        // TODO: Add test case for this.
                        self.childrens[index - 1].remove(key)
                    }
                }
            }
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

    pub fn remove(&mut self, key: &u32) -> Option<u32> {
        if let Some(node) = self.root.as_mut() {
            let result = node.remove(key);

            if node.keys.is_empty() {
                self.root = Some(node.childrens.remove(0));
            }

            result
        } else {
            None
        }
    }

    pub fn get(&self, key: &u32) -> Option<&u32> {
        if let Some(node) = &self.root {
            node.search(key)
        } else {
            None
        }
    }

    pub fn print(&self) {
        if let Some(node) = &self.root {
            let mut queue = VecDeque::new();
            queue.push_front(node);
            let mut visited_child = 0;
            let mut num_of_childs = 1;
            let mut next_to_visit = 0;

            while let Some(node) = queue.pop_back() {
                print!(" {:?} ", node.keys);
                // println!("{:?}: {:?}", node.keys, node.childrens);
                visited_child += 1;

                for c in &node.childrens {
                    queue.push_front(c);
                    next_to_visit += 1;
                }

                if num_of_childs == visited_child {
                    println!("");
                    visited_child = 0;
                    num_of_childs = next_to_visit;
                    next_to_visit = 0;
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::BTree;

    #[test]
    fn basics() {
        let mut tree = BTree::new();
        tree.insert(2);
        tree.insert(7);
        tree.insert(8);
        tree.insert(9);
        tree.insert(4);
        tree.insert(6);
        tree.insert(1);
        tree.insert(5);
        tree.insert(3);
        tree.insert(10);
        tree.insert(11);
        tree.insert(14);
        tree.insert(16);
        tree.insert(17);
        tree.insert(18);
        tree.insert(19);
        tree.insert(20);
        tree.insert(21);
        tree.insert(22);
        tree.insert(23);
        tree.insert(24);
        tree.insert(25);
        tree.insert(30);

        assert_eq!(tree.get(&2), Some(&2));
        assert_eq!(tree.get(&7), Some(&7));
        assert_eq!(tree.get(&8), Some(&8));
        assert_eq!(tree.get(&9), Some(&9));
        assert_eq!(tree.get(&5), Some(&5));
        assert_eq!(tree.get(&10), Some(&10));
        assert_eq!(tree.get(&4), Some(&4));
        assert_eq!(tree.get(&12), None);
        assert_eq!(tree.get(&5), Some(&5));

        tree.remove(&7);
        // Previously, childrens linkage broke here.
        tree.remove(&16);
        tree.remove(&1);
        tree.remove(&18);

        // Fixed!
        // tree.remove(&24);
        // tree.print();
        // tree.remove(&23);
        // tree.print();

        // Fixed!
        // tree.remove(&6);
        // tree.remove(&19);
        // tree.print();

        // Fixed!
        // tree.remove(&14);
        // tree.print();

        // Fixed!
        tree.remove(&21);
        tree.print();
    }

    #[test]
    fn merge_child_before_swapping_left_child_bigget_value() {
        let mut tree = BTree::new();
        tree.insert(10);
        tree.insert(11);
        tree.insert(14);
        tree.insert(16);
        tree.insert(17);
        tree.insert(18);
        tree.insert(19);
        tree.insert(20);
        tree.insert(21);
        tree.insert(22);
        tree.insert(23);
        tree.insert(24);
        tree.insert(25);
        tree.insert(30);
        tree.insert(1);
        tree.insert(2);
        tree.insert(15);
        tree.insert(13);
        tree.insert(12);
        tree.insert(26);
        tree.insert(27);
        tree.insert(28);
        tree.insert(29);

        tree.print();
        assert_eq!(tree.remove(&16), Some(16));
        tree.print();
    }

    #[test]
    fn merge_child_before_swapping_right_child_smallest_value() {
        let mut tree = BTree::new();
        tree.insert(2);
        tree.insert(7);
        tree.insert(8);
        tree.insert(9);
        tree.insert(4);
        tree.insert(6);
        tree.insert(1);
        tree.insert(5);
        tree.insert(3);
        tree.insert(10);
        tree.insert(11);
        tree.insert(14);
        tree.insert(16);
        tree.insert(17);
        tree.insert(18);
        tree.insert(19);
        tree.insert(20);
        tree.insert(21);
        tree.insert(22);
        tree.insert(23);
        tree.insert(24);
        tree.insert(25);
        tree.insert(30);

        tree.print();
        assert_eq!(tree.remove(&18), Some(18));
        tree.print();

        // Steal from right, merge, and remove
        assert_eq!(tree.remove(&16), Some(16));
        tree.print();
    }

    #[test]
    fn case_3a() {
        let mut tree = BTree::new();
        tree.insert(2);
        tree.insert(7);
        tree.insert(8);
        tree.insert(9);
        tree.insert(4);
        tree.insert(6);
        tree.insert(1);
        tree.insert(5);
        tree.insert(3);
        tree.insert(10);
        tree.insert(11);
        tree.insert(14);
        tree.insert(16);
        tree.insert(17);
        tree.insert(18);
        tree.insert(19);
        tree.insert(20);
        tree.insert(21);
        tree.insert(22);
        tree.insert(23);
        tree.insert(24);
        tree.insert(25);
        tree.insert(30);
        tree.insert(31);
        tree.insert(32);
        tree.insert(33);
        tree.insert(34);
        tree.insert(35);

        assert_eq!(tree.remove(&7), Some(7));
    }

    #[test]
    fn case_3b() {
        let mut tree = BTree::new();
        tree.insert(2);
        tree.insert(7);
        tree.insert(8);
        tree.insert(9);
        tree.insert(4);
        tree.insert(6);
        tree.insert(1);
        tree.insert(5);
        tree.insert(3);
        tree.insert(10);
        tree.insert(11);
        tree.insert(14);
        tree.insert(16);
        tree.insert(17);
        tree.insert(18);
        tree.insert(19);
        tree.insert(20);
        tree.insert(21);
        tree.insert(22);
        tree.insert(23);
        tree.insert(24);
        tree.insert(25);
        tree.insert(30);
        tree.insert(31);
        tree.insert(32);
        tree.insert(33);
        tree.insert(34);
        tree.insert(35);

        assert_eq!(tree.remove(&18), Some(18));
    }

    #[test]
    fn delete_key_on_root_node_with_internal_nodes_case_a() {
        let mut tree = BTree::new();
        tree.insert(2);
        tree.insert(7);
        tree.insert(8);
        tree.insert(9);
        tree.insert(4);
        tree.insert(6);
        tree.insert(1);
        tree.insert(5);
        tree.insert(3);
        tree.insert(10);
        tree.insert(11);
        tree.insert(14);

        assert_eq!(tree.remove(&7), Some(7));
    }

    #[test]
    fn delete_key_on_root_node_with_internal_nodes_case_b() {
        let mut tree = BTree::new();
        tree.insert(2);
        tree.insert(7);
        tree.insert(8);
        tree.insert(9);
        tree.insert(4);
        tree.insert(6);
        tree.insert(1);
        tree.insert(5);
        tree.insert(3);
        tree.insert(10);
        tree.insert(11);
        tree.insert(14);
        tree.insert(16);
        tree.insert(17);

        assert_eq!(tree.remove(&7), Some(7));
    }

    #[test]
    fn delete_key_on_root_node() {
        let mut tree = BTree::new();
        tree.insert(2);
        tree.insert(7);
        tree.insert(8);

        assert_eq!(tree.remove(&7), Some(7));
        assert_eq!(tree.remove(&8), Some(8));
        assert_eq!(tree.remove(&1), None);
        assert_eq!(tree.remove(&8), None);
    }

    #[test]
    fn delete_leaf_on_two_leaf_node() {
        let mut tree = BTree::new();
        tree.insert(2);
        tree.insert(7);
        tree.insert(8);
        tree.insert(9);
        tree.insert(4);

        assert_eq!(tree.remove(&4), Some(4));
        assert_eq!(tree.remove(&9), Some(9));
        assert_eq!(tree.remove(&5), None);
    }

    #[test]
    fn delete_key_on_internal_node_case_a() {
        let mut tree = BTree::new();
        tree.insert(2);
        tree.insert(7);
        tree.insert(8);
        tree.insert(9);
        tree.insert(4);
        tree.insert(6);
        tree.insert(1);

        // Actual case a
        assert_eq!(tree.remove(&4), Some(4));
    }

    #[test]
    fn delete_key_on_internal_node_case_b() {
        let mut tree = BTree::new();
        tree.insert(2);
        tree.insert(7);
        tree.insert(8);
        tree.insert(9);
        tree.insert(4);
        tree.insert(6);
        tree.insert(1);
        tree.insert(5);

        assert_eq!(tree.remove(&2), Some(2));

        // Actual case b
        assert_eq!(tree.remove(&4), Some(4));
    }

    #[test]
    fn delete_key_on_internal_node_case_c() {
        let mut tree = BTree::new();
        tree.insert(2);
        tree.insert(7);
        tree.insert(8);
        tree.insert(9);
        tree.insert(4);
        tree.insert(6);
        tree.insert(1);
        tree.insert(5);

        assert_eq!(tree.remove(&2), Some(2));
        assert_eq!(tree.remove(&5), Some(5));

        // Actual case c
        assert_eq!(tree.remove(&4), Some(4));
    }
}
