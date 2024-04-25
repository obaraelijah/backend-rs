use std::collections::VecDeque;

const DEBUG: bool = false;
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

    pub fn remove(&mut self, key: &u32, max_degree: usize) -> Option<u32> {
        // println!("--- remove {key} from {:?}", self);
        let (index, result) = match self.keys.binary_search(key) {
            Ok(index) => {
                if self.is_leaf {
                    let value = self.values.remove(index);
                    self.keys.remove(index);
                    (Some(index), Some(value))
                } else {
                    (
                        Some(index + 1),
                        self.remove_from_internals(index, max_degree),
                    )
                }
            }
            Err(index) => {
                if self.is_leaf {
                    (None, None)
                } else {
                    (Some(index), self.childrens[index].remove(key, max_degree))
                }
            }
        };

        if let Some(index) = index {
            // println!("--- keys: {:?}, index: {index}", self.keys);
            self.rebalance(index, max_degree);
        }
        result
    }

    pub fn remove_from_internals(&mut self, index: usize, max_degree: usize) -> Option<u32> {
        if DEBUG {
            println!("--- remove_from_internals");
        }
        let key = self.keys[index];
        self.keys.remove(index);

        if DEBUG {
            println!(
                "self: {:?}, child: {:?}, index: {index}",
                self, self.childrens
            );
        }
        let min_key = self.min_key(max_degree);
        let child_key = self.childrens[index + 1].keys.len();
        let result = self.childrens[index + 1].remove(&key, max_degree);

        if child_key == min_key {
            if self.childrens[index + 1].is_leaf {
                // println!("Case 2b: {:?}", self.childrens[index + 1]);
                self.fill_with_immediate_sibling(index, max_degree);
            }
        } else {
            // println!("Case 2a: {:?}", self.childrens[index + 1]);
            self.fill_with_inorder_successor(index);
        };

        // This mean that the actual remove happen at self children
        // children. Hence, we want to pick an inorder successor to
        // replace the key we just removed.
        if self.childrens.len() > index + 1 && !self.childrens[index + 1].is_leaf {
            // println!("Case 2c: {:?}", self.childrens);

            if child_key == min_key {
                self.fill_with_inorder_successor(index);
            }
        }

        result
    }

    pub fn fill_with_immediate_sibling(&mut self, index: usize, max_degree: usize) {
        if DEBUG {
            println!("--- fill_with_immediate_sibling");
        }
        let min_key = self.min_key(max_degree);

        if self.childrens[index].keys.len() > min_key {
            let left_sibling = self.childrens.get_mut(index).unwrap();
            let steal_key = left_sibling.keys.pop().unwrap();
            let steal_value = left_sibling.values.pop().unwrap();
            // println!("Steal {steal_key} from left sibling {:?}...", left_sibling);
            self.keys.insert(index, steal_key);
            self.childrens[index + 1].keys.insert(0, steal_key);
            self.childrens[index + 1].values.insert(0, steal_value);
        } else if self.childrens[index + 1].keys.len() > min_key {
            println!("------------------- to handle");
        } else {
            // println!("Case 3 internal");
            self.childrens.remove(index + 1);
        }
    }

    pub fn fill_with_inorder_successor(&mut self, index: usize) {
        if DEBUG {
            // println!("--- fill_with_inorder_successor");
        }

        let node = &self.childrens[index + 1];
        let mut indexes = vec![];
        for (i, n) in node.childrens.iter().enumerate() {
            if n.keys.is_empty() {
                indexes.push(i);
            }
        }

        let mut_node = &mut self.childrens[index + 1];
        let mut removed_elem = 0;
        for i in indexes {
            mut_node.childrens.remove(i - removed_elem);
            removed_elem += 1;
        }

        let mut successor = &self.childrens[index + 1];
        while !successor.childrens.is_empty() {
            successor = &successor.childrens[0];
        }

        self.keys.insert(index, successor.keys[0]);

        // We need to see if our child internal node contain the key
        // that we have just inserted. If yes, remove it.
        if !self.childrens[index + 1].is_leaf {
            if let Ok(key_index) = self.childrens[index + 1]
                .keys
                .binary_search(&successor.keys[0])
            {
                self.childrens[index + 1].keys.remove(key_index);
            }
        }
    }

    pub fn find_indexes_involved(&self, mut index: usize) -> (usize, usize, usize) {
        if DEBUG {
            println!("keys: {:?}", self.keys);
            println!("index: {index}, child_key: {}", self.childrens.len());
        }

        while index >= self.childrens.len() {
            index -= 1;
        }

        for (i, n) in self.childrens.iter().enumerate() {
            if n.keys.is_empty() {
                index = i;
                break;
            }
        }

        let mut left_index = index;
        let mut right_index = index;

        if index != 0 {
            left_index -= 1;
        }

        if index != self.childrens.len() - 1 {
            right_index += 1;
        }

        (index, left_index, right_index)
    }

    pub fn rebalance(&mut self, index: usize, max_degree: usize) {
        if !self.is_leaf && !self.keys.is_empty() {
            if DEBUG {
                println!("--- rebalance");
                println!("self: {:?}, child: {:?}", self, self.childrens);
            }

            let (index, left_index, right_index) = self.find_indexes_involved(index);

            if DEBUG {
                println!("left_index: {left_index}, index: {index}, right_index: {right_index}");
            }

            if self.childrens[index].keys.is_empty() {
                if !self.childrens[index].is_leaf {
                    self.rebalance_internal_node(index, right_index, left_index, max_degree);
                } else {
                    if DEBUG {
                        println!("keys: {:?}", self.keys);
                        println!(
                            "right keys: {:?}, child_key: {:?}, left keys: {:?}",
                            self.childrens[right_index].keys,
                            self.childrens[index].keys,
                            self.childrens[left_index].keys
                        );
                    }

                    let min_key = self.min_key(max_degree);
                    if self.childrens[left_index].keys.len() < min_key
                        && self.childrens[right_index].keys.len() < min_key
                    {
                        println!("to handle");
                    } else {
                        // println!("Remove empty child");
                        self.keys.remove(index);
                        self.childrens.remove(index);
                    }
                }
            }
            if DEBUG {
                println!("-----\n");
            }
        }
    }

    pub fn rebalance_internal_node(
        &mut self,
        index: usize,
        right_index: usize,
        left_index: usize,
        max_degree: usize,
    ) {
        let min_key = self.min_key(max_degree);

        if DEBUG {
            println!(
                "parent: {:?}, left child: {:?}, right child: {:?}",
                self.keys, self.childrens[left_index].keys, self.childrens[right_index].keys,
            );
        }

        if self.childrens[index].keys.len() <= min_key {
            if self.keys.len() <= min_key {
                // println!("merge right and left siblings with parents");
                let mut left = self.childrens.remove(left_index);
                let mut right = self.childrens.remove(left_index);
                left.childrens.append(&mut right.childrens);
                left.keys.append(&mut self.keys);
                left.keys.append(&mut right.keys);

                self.childrens.push(left);
            } else if index != left_index && self.childrens[left_index].keys.len() > min_key {
                // Parent steal from left child. Right child steal key and child
                // from left child.
                let parent_key = self.keys.remove(left_index);
                // println!("Right child steal {parent_key} from parent...");
                let right = &mut self.childrens[index];
                right.keys.push(parent_key);

                let left = &mut self.childrens[left_index];
                let steal_key = left.keys.pop().unwrap();
                // println!("Parent steal {steal_key} from left_child...");
                let steal_child = left.childrens.pop().unwrap();
                self.keys.insert(left_index, steal_key);

                let right = &mut self.childrens[index];
                right.childrens.insert(0, steal_child);
            } else if index != right_index && self.childrens[right_index].keys.len() > min_key {
                // Parent steal from right child. Left child steal key and child
                // from right child.
                let parent_key = self.keys.remove(index);
                // println!("Left child steal {parent_key} from parent...");
                let left = &mut self.childrens[index];
                left.keys.push(parent_key);

                let right = &mut self.childrens[right_index];
                let steal_key = right.keys.remove(0);
                // println!("Parent steal {steal_key} from right child...");
                let steal_child = right.childrens.remove(0);
                self.keys.insert(index, steal_key);

                let left = &mut self.childrens[index];
                left.childrens.push(steal_child);
            } else {
                // println!("merge right and left siblings, remove key from parent");

                let parent_key = if index > 0 {
                    self.keys.remove(index - 1)
                } else {
                    self.keys.remove(index)
                };

                let mut left = self.childrens.remove(left_index);
                let mut right = self.childrens.remove(left_index);

                // println!("{:?}, {:?}", left, right);

                left.keys.push(parent_key);
                left.keys.append(&mut right.keys);
                left.childrens.append(&mut right.childrens);

                self.childrens.insert(left_index, left);
            }
        } else if self.childrens[left_index].keys.len() <= min_key {
            // println!("Steal from parent to merge with right siblings...");
            let parent_key = self.keys.remove(left_index);

            let mut left = self.childrens.remove(left_index);
            // - 1 because we remove left above.
            let right = &mut self.childrens[right_index - 1];

            // Combine childrens of right and left.
            left.childrens.append(&mut right.childrens);
            right.childrens = left.childrens;

            // Steal key from parent.
            right.keys.insert(0, parent_key);
        } else if self.childrens[right_index].keys.len() <= min_key {
            // Parent steal from left child. Right child steal key and child
            // from left child.
            let parent_key = self.keys.remove(right_index - 1);
            // println!("Right child steal {parent_key} from parent...");
            let right = &mut self.childrens[right_index];
            right.keys.push(parent_key);

            let left = &mut self.childrens[left_index];
            let steal_key = left.keys.pop().unwrap();
            // println!("Parent steal {steal_key} from left_child...");
            let steal_child = left.childrens.pop().unwrap();
            self.keys.insert(right_index - 1, steal_key);

            let right = &mut self.childrens[right_index];
            right.childrens.insert(0, steal_child);
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
            max_degree,
        };

        for i in numbers {
            tree.insert(i);
        }

        tree
    }

    pub fn insert(&mut self, key: u32) {
        if let Some(node) = self.root.as_mut() {
            node.insert_non_full(key, self.max_degree);

            if node.keys.len() == self.max_degree {
                let mut new_root = Node::new(false);
                new_root.childrens.push(self.root.take().unwrap());
                new_root.split_child(0, self.max_degree);
                self.root = Some(new_root);
            }
        } else {
            let mut node = Node::new(true);
            node.insert_non_full(key, self.max_degree);
            self.root = Some(node);
        }
    }

    pub fn remove(&mut self, key: &u32) -> Option<u32> {
        if let Some(node) = self.root.as_mut() {
            let result = node.remove(key, self.max_degree);

            if node.keys.is_empty() {
                self.root = node.childrens.pop()
            }

            result
        } else {
            None
        }
    }

    pub fn get(&self, key: &u32) -> Option<&u32> {
        self.root.as_ref().map_or(None, |node| node.search(key))
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
    use super::BPlusTree;

    #[test]
    fn get_on_empty_tree() {
        let tree = BPlusTree::new(vec![], 4);
        assert_eq!(tree.get(&2), None);
    }

    #[test]
    fn insert_on_root_node() {
        let mut tree = BPlusTree::new(vec![], 4);

        tree.insert(1);
        tree.insert(2);
        tree.insert(3);

        assert_eq!(tree.get(&1), Some(&1));
        assert_eq!(tree.get(&2), Some(&2));
        assert_eq!(tree.get(&3), Some(&3));
    }

    #[test]
    fn insert_and_split_on_root_node() {
        let mut tree = BPlusTree::new(vec![7, 10, 15], 4);
        tree.insert(8);

        assert_eq!(tree.get(&8), Some(&8));
        assert_eq!(tree.get(&18), None);
    }

    #[test]
    fn insert_on_leaf_node() {
        let mut tree = BPlusTree::new(vec![7, 10, 15, 8], 4);
        tree.insert(11);
        assert_eq!(tree.get(&11), Some(&11));
    }

    #[test]
    fn insert_and_split_on_leaf_node() {
        let mut tree = BPlusTree::new(vec![7, 10, 15, 8, 11], 4);

        tree.insert(12);
        assert_eq!(tree.get(&12), Some(&12));
        assert_eq!(tree.get(&7), Some(&7));
        assert_eq!(tree.get(&8), Some(&8));
    }

    #[test]
    fn insert_and_split_recursively_on_level_3_leaf_node() {
        let vec = vec![7, 10, 15, 8, 11, 12, 19, 25, 30];
        let mut tree = BPlusTree::new(vec.clone(), 4);

        tree.insert(49);
        assert_eq!(tree.get(&49), Some(&49));

        for v in vec {
            assert_eq!(tree.get(&v), Some(&v));
        }
    }

    #[test]
    fn insert_and_split_is_reasign_to_the_right_spot() {
        let vec = vec![7, 10, 15, 8, 11, 12, 19, 25, 30, 49, 69, 90, 59];
        let mut tree = BPlusTree::new(vec.clone(), 4);

        tree.insert(41);
        assert_eq!(tree.get(&41), Some(&41));

        for v in vec {
            assert_eq!(tree.get(&v), Some(&v));
        }
    }

    #[test]
    fn insert_and_split_on_existing_internal_node() {
        let vec = vec![7, 10, 15, 8, 11, 12, 19, 25, 30, 49, 69, 90, 59, 41, 45];
        let mut tree = BPlusTree::new(vec.clone(), 4);

        tree.insert(42);
        assert_eq!(tree.get(&42), Some(&42));
        for v in vec {
            assert_eq!(tree.get(&v), Some(&v));
        }
    }

    #[test]
    fn insert_and_split_on_level_4_leaf_node() {
        let vec = vec![
            7, 10, 15, 8, 11, 12, 19, 25, 30, 49, 69, 90, 59, 41, 45, 42, 1, 4, 50, 52, 5, 6, 9,
            23, 29, 26, 34,
        ];
        let mut tree = BPlusTree::new(vec.clone(), 4);

        tree.insert(35);
        assert_eq!(tree.get(&35), Some(&35));

        for v in vec {
            assert_eq!(tree.get(&v), Some(&v));
        }
    }

    #[test]
    fn insert_and_split_on_level_5_leaf_node() {
        let vec: Vec<u32> = (1..82).collect();
        let mut tree = BPlusTree::new(vec.clone(), 4);

        tree.insert(82);
        assert_eq!(tree.get(&82), Some(&82));

        for v in vec {
            assert_eq!(tree.get(&v), Some(&v));
        }
    }

    #[test]
    fn delete_key_on_root_node() {
        let mut tree = BPlusTree::new(vec![2, 7, 8], 4);

        assert_eq!(tree.remove(&7), Some(7));
        assert_eq!(tree.remove(&8), Some(8));
        assert_eq!(tree.remove(&1), None);
        assert_eq!(tree.remove(&8), None);
    }

    #[test]
    fn delete_key_case1a() {
        let mut vec = vec![2, 7, 8, 9, 4, 6, 1, 5, 3];
        let mut tree = BPlusTree::new(vec.clone(), 4);

        assert_eq!(tree.remove(&7), Some(7));
        assert_eq!(tree.get(&7), None);

        vec.remove(1);
        for v in vec {
            assert_eq!(tree.get(&v), Some(&v));
        }
    }

    #[test]
    fn delete_key_case1b() {
        let mut vec = vec![15, 25, 35, 5, 45, 20, 30, 55, 40];
        let mut tree = BPlusTree::new(vec.clone(), 3);

        assert_eq!(tree.remove(&5), Some(5));

        vec.retain(|&x| x != 5);
        for v in vec {
            assert_eq!(tree.get(&v), Some(&v));
        }
    }

    #[test]
    fn delete_key_case2a() {
        let mut vec = vec![15, 25, 35, 5, 45, 20, 30, 55, 40];
        let mut tree = BPlusTree::new(vec.clone(), 3);
        tree.remove(&40);
        tree.remove(&5);

        assert_eq!(tree.remove(&45), Some(45));
        assert_eq!(tree.get(&45), None);

        vec.retain(|&x| x != 40 && x != 5 && x != 45);
        for v in vec {
            assert_eq!(tree.get(&v), Some(&v));
        }
    }

    #[test]
    fn delete_key_case2b() {
        let mut vec = vec![2, 7, 8, 9, 4, 6, 1, 5, 3];
        let mut tree = BPlusTree::new(vec.clone(), 4);
        tree.remove(&7);

        assert_eq!(tree.remove(&6), Some(6));
        assert_eq!(tree.get(&6), None);

        vec.retain(|&x| x != 7 && x != 6);
        for v in vec {
            assert_eq!(tree.get(&v), Some(&v));
        }
    }

    #[test]
    fn delete_key_case2c() {
        let mut vec = vec![
            7, 8, 9, 4, 6, 1, 5, 3, 10, 11, 14, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 30,
        ];
        let mut tree = BPlusTree::new(vec.clone(), 4);
        tree.remove(&24);

        assert_eq!(tree.remove(&23), Some(23));
        assert_eq!(tree.get(&23), None);

        vec.retain(|&x| x != 24 && x != 23);
        for v in vec {
            assert_eq!(tree.get(&v), Some(&v));
        }
    }

    #[test]
    fn delete_key_case3() {
        let vec = vec![15, 25, 35, 5, 45, 20, 30, 55, 40];
        let mut tree = BPlusTree::new(vec.clone(), 3);
        tree.remove(&40);
        tree.remove(&5);
        tree.remove(&45);
        tree.remove(&35);
        tree.remove(&25);

        assert_eq!(tree.remove(&55), Some(55));
        assert_eq!(tree.get(&55), None);

        let vec = vec![15, 20, 30];
        for v in vec {
            assert_eq!(tree.get(&v), Some(&v));
        }
    }

    #[test]
    fn delete_key_at_leaf_node_that_require_merge_and_delete_from_internal() {
        // Delete 3 from:
        // [7, 13]
        // [4, 5]  [9, 11]  [15, 17]
        // [3]  [4]  [5, 6]  [7, 8]  [9, 10]  [11, 12]  [13, 14]  [15, 16]  [17, 18, 19]
        //
        // Become:
        // [7, 13]
        // [4, 5]  [9, 11]  [15, 17]
        // []  [4]  [5, 6]  [7, 8]  [9, 10]  [11, 12]  [13, 14]  [15, 16]  [17, 18, 19]
        //
        // Hence, we need to merge node [] and [4], with that our parent has one less child,
        // and hence we need to remove 4 from the parent keys.
        // [7, 13]
        // [5]  [9, 11]  [15, 17]
        // [4]  [5, 6]  [7, 8]  [9, 10]  [11, 12]  [13, 14]  [15, 16]  [17, 18, 19]
        let mut vec: Vec<u32> = (1..20).collect();
        let mut tree = BPlusTree::new(vec.clone(), 4);
        tree.remove(&1);
        tree.remove(&2);

        assert_eq!(tree.remove(&3), Some(3));
        assert_eq!(tree.get(&3), None);

        vec.retain(|&x| x != 1 && x != 2 && x != 3);
        for v in &vec {
            assert_eq!(tree.get(v), Some(v));
        }
    }

    #[test]
    fn delete_key_at_leaf_node_that_require_to_get_key_from_parent_and_steal_sibling_child() {
        // Delete 5 from:
        // [7, 13]
        // [6]  [9, 11]  [15, 17]
        // [5]  [6]  [7, 8]  [9, 10]  [11, 12]  [13, 14]  [15, 16]  [17, 18, 19]
        //
        // Become:
        // [7, 13]
        // [6]  [9, 11]  [15, 17]
        // []  [6]  [7, 8]  [9, 10]  [11, 12]  [13, 14]  [15, 16]  [17, 18, 19]
        //
        // Hence, we need to merge [], [6], by getting from parent and parent steal from
        // right sibling, since now our right sibling have less key, we also need to
        // steal their child:
        // [7, 13]
        // [6]  [9, 11]  [15, 17]
        // []  [6]  [7, 8]  [9, 10]  [11, 12]  [13, 14]  [15, 16]  [17, 18, 19]
        //
        // and after merge:
        // [9, 13]
        // [7]  [11]  [15, 17]
        // [6]  [7, 8]  [9, 10]  [11, 12]  [13, 14]  [15, 16]  [17, 18, 19]
        let mut vec: Vec<u32> = (1..20).collect();
        let mut tree = BPlusTree::new(vec.clone(), 4);
        tree.remove(&1);
        tree.remove(&2);
        tree.remove(&3);
        tree.remove(&4);

        assert_eq!(tree.remove(&5), Some(5));
        assert_eq!(tree.get(&5), None);

        vec.retain(|x| ![1, 2, 3, 4, 5].contains(x));
        for v in &vec {
            assert_eq!(tree.get(v), Some(v));
        }
    }

    #[test]
    fn delete_key_at_leaf_node_that_require_get_key_from_parent_to_combine_with_sibling_child() {
        // Remove 7 from:
        // [9, 13]
        // [8]  [11]  [15, 17]
        // [7]  [8]  [9, 10]  [11, 12]  [13, 14]  [15, 16]  [17, 18, 19]
        //
        // Become:
        // [9, 13]
        // [8]  [11]  [15, 17]
        // []  [8]  [9, 10]  [11, 12]  [13, 14]  [15, 16]  [17, 18, 19]
        //
        // Since, we can't steal from right siblings as it only have one key,
        // we steal from our parent. This mean that parent will have one less child as
        // welll as we need to merge our left and right siblings:
        // [13]
        // [9, 11]  [15, 17]
        // [8]  [9, 10]  [11, 12]  [13, 14]  [15, 16]  [17, 18, 19]
        let mut vec: Vec<u32> = (1..20).collect();
        let mut tree = BPlusTree::new(vec.clone(), 4);
        tree.remove(&1);
        tree.remove(&2);
        tree.remove(&3);
        tree.remove(&4);
        tree.remove(&5);
        tree.remove(&6);

        assert_eq!(tree.remove(&7), Some(7));
        assert_eq!(tree.get(&7), None);

        vec.retain(|x| ![1, 2, 3, 4, 5, 6, 7].contains(x));
        for v in &vec {
            assert_eq!(tree.get(v), Some(v));
        }
    }

    #[test]
    fn delete_all_keys_from_left_to_right() {
        let mut vec: Vec<u32> = (1..200).collect();
        let mut tree = BPlusTree::new(vec.clone(), 3);

        vec.retain(|&x| x != 199);
        for &v in &vec {
            assert_eq!(tree.remove(&v), Some(v));
        }

        assert_eq!(tree.remove(&199), Some(199));
        assert_eq!(tree.get(&199), None);

        for v in &vec {
            assert_eq!(tree.get(v), None);
        }
    }

    #[test]
    fn delete_keys_on_leaf_node_that_need_to_steal_from_both_sibling_and_parent() {
        // Delete 14 from:
        // [7, 13]
        // [3, 5]  [9, 11]  [14]
        // [1, 2]  [3, 4]  [5, 6]  [7, 8]  [9, 10]  [11, 12]  [13]  [14]
        //
        // become:
        // [7, 13]
        // [3, 5]  [9, 11]  []
        // [1, 2]  [3, 4]  [5, 6]  [7, 8]  [9, 10]  [11, 12]  [13]  []
        //
        // since, parent and sibling have enough keys, right sibling steal key from parent.
        // Parent will then steal key from the siblings. Since left sibling has child,
        // right sibling will need to steal the child from left sibling.
        //
        // [7, 11]
        // [3, 5]  [9]  [13]
        // [1, 2]  [3, 4]  [5, 6]  [7, 8]  [9, 10]  [11, 12]  [13]
        let mut vec: Vec<u32> = (1..20).collect();
        let mut tree = BPlusTree::new(vec.clone(), 4);
        tree.remove(&19);
        tree.remove(&18);
        tree.remove(&17);
        tree.remove(&16);
        tree.remove(&15);

        assert_eq!(tree.remove(&14), Some(14));
        assert_eq!(tree.get(&14), None);

        vec.retain(|x| ![19, 18, 17, 16, 15, 14].contains(x));
        for v in &vec {
            assert_eq!(tree.get(v), Some(v));
        }
    }

    #[test]
    fn delete_all_keys_from_right_to_left() {
        let mut vec: Vec<u32> = (1..20).collect();
        let mut tree = BPlusTree::new(vec.clone(), 4);

        vec.retain(|&x| x != 1);
        for &v in vec.iter().rev() {
            assert_eq!(tree.remove(&v), Some(v));
        }

        assert_eq!(tree.remove(&1), Some(1));
        assert_eq!(tree.get(&1), None);

        for v in &vec {
            assert_eq!(tree.get(v), None);
        }
    }

    #[test]
    fn random_test_case_1() {
        let mut vec: Vec<u32> = (1..20).collect();
        let mut tree = BPlusTree::new(vec.clone(), 4);
        let deletes = vec![18, 16, 15, 13, 6, 17, 4, 3, 2, 11, 7, 9, 12];

        for v in &deletes {
            assert_eq!(tree.remove(v), Some(*v));
        }

        assert_eq!(tree.remove(&14), Some(14));
        assert_eq!(tree.get(&14), None);
        assert_eq!(tree.remove(&19), Some(19));
        assert_eq!(tree.get(&19), None);

        vec.retain(|x| !deletes.contains(x) && *x != 14 && *x != 19);
        for v in &vec {
            assert_eq!(tree.get(v), Some(v));
        }
    }

    #[test]
    fn random_test_case_2() {
        let mut vec: Vec<u32> = (1..20).collect();
        let mut tree = BPlusTree::new(vec.clone(), 4);
        let deletes = vec![
            16, 11, 12, 6, 17, 4, 15, 18, 13, 3, 14, 10, 2, 9, 19, 1, 5, 7, 8,
        ];

        for v in &deletes {
            assert_eq!(tree.remove(v), Some(*v));
        }

        vec.retain(|x| !deletes.contains(x));
        for v in &vec {
            assert_eq!(tree.get(v), None);
        }
    }

    #[test]
    fn random_test_case_3() {
        let mut vec: Vec<u32> = (1..20).collect();
        let mut tree = BPlusTree::new(vec.clone(), 4);
        let deletes = vec![1, 5, 19, 18, 6, 3, 2, 10, 8, 12, 14, 17, 13];
        let to_deletes = vec![16, 15, 7, 11, 4];

        for v in &deletes {
            assert_eq!(tree.remove(v), Some(*v));
        }

        assert_eq!(tree.remove(&9), Some(9));
        assert_eq!(tree.get(&9), None);

        for v in &to_deletes {
            assert_eq!(tree.remove(v), Some(*v));
        }

        vec.retain(|x| !deletes.contains(x) && !to_deletes.contains(x));
        for v in &vec {
            assert_eq!(tree.get(v), None);
        }
    }

    #[test]
    fn random_test_case_4() {
        let mut vec: Vec<u32> = (1..20).collect();
        let mut tree = BPlusTree::new(vec.clone(), 4);
        let deletes = vec![11, 10, 12, 18, 7, 16, 14, 19, 2];
        let to_deletes = vec![1, 5, 13, 8, 4, 15, 6, 3, 17];

        for v in &deletes {
            assert_eq!(tree.remove(v), Some(*v));
        }

        assert_eq!(tree.remove(&9), Some(9));
        assert_eq!(tree.get(&9), None);

        for v in &to_deletes {
            assert_eq!(tree.remove(v), Some(*v));
        }

        vec.retain(|x| !deletes.contains(x) && !to_deletes.contains(x));
        for v in &vec {
            assert_eq!(tree.get(v), None);
        }
    }

    // Use to generate random test case.
    //
    // If a test failed, we would add the test case manually.
    // as part of our test suite.
    use rand::seq::SliceRandom;
    use rand::thread_rng;
    #[test]
    fn delete_all_keys_randomly() {
        for _i in 0..1000 {
            let mut vec: Vec<u32> = (1..200).collect();
            let mut tree = BPlusTree::new(vec.clone(), 4);
            vec.shuffle(&mut thread_rng());

            // println!("{i}: {:?}", vec);
            for &v in &vec {
                assert_eq!(tree.remove(&v), Some(v));
            }
        }
    }
}