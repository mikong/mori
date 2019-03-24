use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
enum Edge {
    Null,
    Link(Rc<RefCell<Node>>),
}

impl Edge {
    pub fn new(key: usize, value: String) -> Self {
        Edge::Link(Rc::new(RefCell::new(Node::new(key, value))))
    }

    pub fn is_null(&self) -> bool {
        match *self {
            Edge::Link(_) => false,
            Edge::Null => true,
        }
    }

    pub fn size(&self) -> usize {
        match self {
            Edge::Link(node) => node.borrow().size,
            Edge::Null => 0,
        }
    }
}

#[derive(Debug)]
pub struct Node {
    key: usize,
    value: String,
    left: Edge,
    right: Edge,
    size: usize,
}

impl Node {
    pub fn new(key: usize, value: String) -> Self {
        Node {
            key,
            value,
            left: Edge::Null,
            right: Edge::Null,
            size: 1,
        }
    }
}

#[derive(Debug)]
pub struct BST {
    root: Edge,
}

impl BST {
    pub fn new() -> Self {
        BST { root: Edge::Null }
    }

    pub fn is_empty(&self) -> bool {
        self.root.size() == 0
    }

    pub fn size(&self) -> usize {
        self.root.size()
    }

    pub fn get(&self, key: usize) -> Option<String> {
        BST::getr(&self.root, key)
    }

    fn getr(x: &Edge, key: usize) -> Option<String> {
        if let Edge::Link(node) = x {
            if key < node.borrow().key {
                return BST::getr(&node.borrow().left, key);
            } else if key > node.borrow().key {
                return BST::getr(&node.borrow().right, key);
            } else {
                return Some(node.borrow().value.clone());
            }
        }
        None
    }

    pub fn put(&mut self, key: usize, value: String) {
        self.root = BST::insert(&self.root, key, value);
    }

    fn insert(x: &Edge, key: usize, value: String) -> Edge {
        if let Edge::Link(node) = x {
            if key < node.borrow().key {
                let new_node = BST::insert(&node.borrow().left, key, value);
                node.borrow_mut().left = new_node;
            } else if key > node.borrow().key {
                let new_node = BST::insert(&node.borrow().right, key, value);
                node.borrow_mut().right = new_node;
            }
            // TODO: if same key, update value
            let size = 1 + node.borrow().left.size() + node.borrow().right.size();
            node.borrow_mut().size = size;
            return Edge::Link(Rc::clone(node));
        }
        // x = Null
        Edge::new(key, value)
    }

    pub fn delete_min(&mut self) {
        self.root = BST::remove_min(&self.root);
    }

    fn remove_min(x: &Edge) -> Edge {
        if let Edge::Link(node) = x {
            if node.borrow().left.is_null() {
                match &node.borrow().right {
                    Edge::Link(node) => return Edge::Link(Rc::clone(node)),
                    Edge::Null => return Edge::Null,
                }
            }
            let new_node = BST::remove_min(&node.borrow().left);
            node.borrow_mut().left = new_node;
            let size = 1 + node.borrow().left.size() + node.borrow().right.size();
            node.borrow_mut().size = size;
            return Edge::Link(Rc::clone(node));
        }
        Edge::Null
    }

    pub fn keys(&self) -> Vec<usize> {
        let mut v = Vec::new();
        BST::inorder(&self.root, &mut v);
        v
    }

    fn inorder(x: &Edge, v: &mut Vec<usize>) {
        if let Edge::Link(node) = x {
            BST::inorder(&node.borrow().left, v);
            v.push(node.borrow().key);
            BST::inorder(&node.borrow().right, v);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_key(cell: &Edge, key: usize) {
        if let Edge::Link(node) = cell {
            assert_eq!(node.borrow().key, key);
        } else {
            panic!("Node can't be None");
        }
    }

    //          8(S)
    //         /    \
    //       3(E)   9(X)
    //      /   \
    //  1(A)     7(R)
    //     \     /
    //    2(C) 5(H)
    //        /   \
    //      4(G)  6(M)
    fn populate_tree(bst: &mut BST) {
        bst.put(8, "S".to_string());
        bst.put(3, "E".to_string());
        bst.put(1, "A".to_string());
        bst.put(7, "R".to_string());
        bst.put(2, "C".to_string());
        bst.put(5, "H".to_string());
        bst.put(9, "X".to_string());
        bst.put(6, "M".to_string());
        bst.put(4, "G".to_string());
    }

    #[test]
    fn create_node() {
        let n = Node::new(1, "a".to_string());
        assert_eq!(n.left.is_null(), true);
        assert_eq!(n.right.is_null(), true);
    }

    #[test]
    fn build_tree() {
        let mut bst = BST::new();
        assert_eq!(bst.root.is_null(), true);

        // New node becomes root
        bst.put(2, "b".to_string());
        check_key(&bst.root, 2);

        // New node becomes left node
        bst.put(1, "a".to_string());
        if let Edge::Link(node) = &bst.root {
            check_key(&node.borrow().left, 1);
        } else {
            panic!("BST must have root");
        }

        // New node becomes right node
        bst.put(3, "c".to_string());
        if let Edge::Link(node) = &bst.root {
            check_key(&node.borrow().right, 3);
        } else {
            panic!("BST must have root");
        }
    }

    #[test]
    fn get_value() {
        let mut bst = BST::new();
        populate_tree(&mut bst);

        assert_eq!(bst.get(8), Some("S".to_string()));
        assert_eq!(bst.get(2), Some("C".to_string()));
        assert_eq!(bst.get(9), Some("X".to_string()));
        assert_eq!(bst.get(5), Some("H".to_string()));
        assert_eq!(bst.get(10), None);

        // TODO: get after delete
    }

    #[test]
    fn inorder_traversal() {
        let mut bst = BST::new();
        populate_tree(&mut bst);

        assert_eq!(bst.keys().len(), 9);
        assert_eq!(bst.keys(), vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn tree_size() {
        let mut bst = BST::new();

        assert_eq!(bst.is_empty(), true);
        assert_eq!(bst.size(), 0);

        populate_tree(&mut bst);

        assert_eq!(bst.is_empty(), false);
        assert_eq!(bst.size(), 9);
    }

    #[test]
    fn remove_min() {
        let mut bst = BST::new();

        // delete min an empty BST
        bst.delete_min();

        populate_tree(&mut bst);

        bst.delete_min();
        assert_eq!(bst.keys(), vec![2, 3, 4, 5, 6, 7, 8, 9]);
        bst.delete_min();
        assert_eq!(bst.keys(), vec![3, 4, 5, 6, 7, 8, 9]);
        bst.delete_min();
        assert_eq!(bst.keys(), vec![4, 5, 6, 7, 8, 9]);
    }
}
