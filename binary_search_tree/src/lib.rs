use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
pub struct Node {
    key: usize,
    value: String,
    left: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(key: usize, value: String) -> Self {
        Node {
            key,
            value,
            left: None,
            right: None,
        }
    }
}

#[derive(Debug)]
pub struct BST {
    root: Option<Rc<RefCell<Node>>>,
}

impl BST {
    pub fn new() -> Self {
        BST { root: None }
    }

    pub fn put(&mut self, key: usize, value: String) {
        self.root = BST::insert(&self.root, key, value);
    }

    fn insert(x: &Option<Rc<RefCell<Node>>>, key: usize, value: String) -> Option<Rc<RefCell<Node>>> {
        if let Some(node) = x {
            if key < node.borrow().key {
                let new_node = BST::insert(&node.borrow().left, key, value);
                node.borrow_mut().left = new_node;
            } else if key > node.borrow().key {
                let new_node = BST::insert(&node.borrow().right, key, value);
                node.borrow_mut().right = new_node;
            }
            // TODO: if same key, update value
            return Some(Rc::clone(node));
        }
        // x = None
        Some(Rc::new(RefCell::new(Node::new(key, value))))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_key(cell: &Option<Rc<RefCell<Node>>>, key: usize) {
        if let Some(node) = cell {
            assert_eq!(node.borrow().key, key);
        } else {
            panic!("Node can't be None");
        }
    }

    #[test]
    fn create_node() {
        let n = Node::new(1, "a".to_string());
        assert_eq!(n.left.is_none(), true);
        assert_eq!(n.right.is_none(), true);
    }

    #[test]
    fn build_tree() {
        let mut bst = BST::new();
        assert_eq!(bst.root.is_none(), true);

        // New node becomes root
        bst.put(2, "b".to_string());
        check_key(&bst.root, 2);

        // New node becomes left node
        bst.put(1, "a".to_string());
        if let Some(node) = &bst.root {
            check_key(&node.borrow().left, 1);
        } else {
            panic!("BST must have root");
        }

        // New node becomes right node
        bst.put(3, "c".to_string());
        if let Some(node) = &bst.root {
            check_key(&node.borrow().right, 3);
        } else {
            panic!("BST must have root");
        }
    }
}
