type Link = Option<Box<Node>>;

struct Node {
    val: i32,
    left: Link,
    right: Link,
}

pub struct BSTree {
    root: Link,
    size: u32,
}

impl Node {
    pub fn print(&self) {
        if let Some(left) = self.left.as_ref() {
            left.print();
        }

        print!("{} ", self.val);

        if let Some(right) = self.right.as_ref() {
            right.print();
        }
    }

    fn remove(mut this: Box<Node>, val: i32) -> Option<Box<Node>> {
        if this.val == val {
            match (this.right.take(), this.left.take()) {
                (None, None) => None,
                (Some(right), Some(mut left)) => {
                    if let Some(mut r) = left.right.as_ref() {
                        let mut current = left.clone();
    
                        while let Some(right) = r.right.as_ref() {
                            current = r.clone();
                            r = right;
                        }
    
                        // Take out our rightmost child from it parents.
                        let mut rightmost = current.right.take().unwrap();
    
                        // If rightmost child have left child,
                        // we will need to move to it to be our parent right child.
                        //
                        // This is because we will be moving our rightmost child
                        // and assign both left and right child from our deleted node.
                        // So, if we didn't reassign, the left child will be abondon
                        // and replaced by our deleted node left child.
                        //
                        // We are not checking for right child because we are getting our
                        // rightmost child, so it is not possible to have right child.
                        if let Some(left) = rightmost.left.take() {
                            current.right = Some(left);
                        }
    
                        // Assign our deleted node right and left child
                        // to our rightmost child.
                        rightmost.left = Some(current);
                        rightmost.right = Some(right);
    
                        Some(rightmost)
                    } else {
                        left.right = Some(right);
                        Some(left)
                    }
                }
                (Some(right), None) => Some(right),
                (None, Some(left)) => Some(left),
            }
        } else if val > this.val {
            if let Some(node) = this.right.take() {
                this.right = remove(node, val);
            }
    
            Some(this)
        } else {
            if let Some(node) = this.left.take() {
                this.left = remove(node, val);
            }
    
            Some(this)
        }
    }
}


impl BSTree {
    pub fn new() -> BSTree {
        BSTree {
            root: None,
            size: 0,
        }
    }

    pub fn get(&self, val: &i32) -> Option<&i32> {
        let mut node = self.root.as_ref();

        while let Some(n) = node {
            if &n.val == val {
                return Some(&n.val);
            }

            if &n.val > val {
                node = n.left.as_ref();
            } else {
                node = n.right.as_ref();
            }
        }

        None
    }

    pub fn remove(&self, val: i32) {
        if let Some(node) => self.root.take() {
            self.root = remove(node, val);
        }
    }

    pub fn print(&self) {
        if let Some(node) = &self.root {
            node.print();
            println!("");
        }
    }
}