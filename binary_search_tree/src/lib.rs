use std::rc::Rc;
use std::cell::RefCell;
use std::cell::Ref;

#[derive(Debug)]
pub struct NodePtr(Rc<RefCell<Node>>);

impl NodePtr {
    pub fn new(key: usize, value: String) -> Self {
        let node = Rc::new(RefCell::new(Node::new(key, value)));
        NodePtr(node)
    }

    pub fn node(&self) -> Ref<Node> {
        self.0.borrow()
    }

    pub fn set_left(&self, node: Option<NodePtr>) {
        self.0.borrow_mut().left = node;
    }

    pub fn set_right(&self, node: Option<NodePtr>) {
        self.0.borrow_mut().right = node;
    }

    pub fn set_size(&self, size: usize) {
        self.0.borrow_mut().size = size;
    }

    pub fn clone(&self) -> Self {
        NodePtr(Rc::clone(&self.0))
    }
}

#[derive(Debug)]
pub struct Node {
    key: usize,
    value: String,
    left: Option<NodePtr>,
    right: Option<NodePtr>,
    size: usize,
}

impl Node {
    pub fn new(key: usize, value: String) -> Self {
        Node {
            key,
            value,
            left: None,
            right: None,
            size: 1,
        }
    }

    pub fn size(node: &Option<NodePtr>) -> usize {
        match node {
            Some(node) => node.0.borrow().size,
            None => 0,
        }
    }
}

#[derive(Debug)]
pub struct BST {
    root: Option<NodePtr>,
}

impl BST {
    pub fn new() -> Self {
        BST { root: None }
    }

    pub fn is_empty(&self) -> bool {
        Node::size(&self.root) == 0
    }

    pub fn size(&self) -> usize {
        Node::size(&self.root)
    }

    pub fn get(&self, key: usize) -> Option<String> {
        BST::getr(&self.root, key)
    }

    fn getr(x: &Option<NodePtr>, key: usize) -> Option<String> {
        if let Some(node_ptr) = x {
            if key < node_ptr.node().key {
                return BST::getr(&node_ptr.node().left, key);
            } else if key > node_ptr.node().key {
                return BST::getr(&node_ptr.node().right, key);
            } else {
                return Some(node_ptr.node().value.clone());
            }
        }
        None
    }

    pub fn put(&mut self, key: usize, value: String) {
        self.root = BST::insert(&self.root, key, value);
    }

    fn insert(x: &Option<NodePtr>, key: usize, value: String) -> Option<NodePtr> {
        if let Some(node_ptr) = x {
            if key < node_ptr.node().key {
                let new_node = BST::insert(&node_ptr.node().left, key, value);
                node_ptr.set_left(new_node);
            } else if key > node_ptr.node().key {
                let new_node = BST::insert(&node_ptr.node().right, key, value);
                node_ptr.set_right(new_node);
            }
            // TODO: if same key, update value
            let size = 1 + Node::size(&node_ptr.node().left) + Node::size(&node_ptr.node().right);
            node_ptr.set_size(size);
            return Some(node_ptr.clone());
        }
        // x = Null
        Some(NodePtr::new(key, value))
    }

    pub fn delete_min(&mut self) {
        self.root = BST::remove_min(&self.root);
    }

    fn remove_min(x: &Option<NodePtr>) -> Option<NodePtr> {
        if let Some(node_ptr) = x {
            if node_ptr.node().left.is_none() {
                match &node_ptr.node().right {
                    Some(node_ptr) => return Some(node_ptr.clone()),
                    None => return None,
                }
            }
            let new_node = BST::remove_min(&node_ptr.node().left);
            node_ptr.set_left(new_node);
            let size = 1 + Node::size(&node_ptr.node().left) + Node::size(&node_ptr.node().right);
            node_ptr.set_size(size);
            return Some(node_ptr.clone());
        }
        None
    }

    pub fn min(&self) -> Option<usize> {
        if let Some(node_ptr) = BST::minimum(&self.root) {
            return Some(node_ptr.node().key)
        }

        None
    }

    fn minimum(x: &Option<NodePtr>) -> Option<NodePtr> {
        if let Some(node_ptr) = x {
            if node_ptr.node().left.is_none() {
                return Some(node_ptr.clone());
            } else {
                return BST::minimum(&node_ptr.node().left);
            }
        }

        None
    }

    pub fn keys(&self) -> Vec<usize> {
        let mut v = Vec::new();
        BST::inorder(&self.root, &mut v);
        v
    }

    fn inorder(x: &Option<NodePtr>, v: &mut Vec<usize>) {
        if let Some(node_ptr) = x {
            BST::inorder(&node_ptr.node().left, v);
            v.push(node_ptr.node().key);
            BST::inorder(&node_ptr.node().right, v);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_key(cell: &Option<NodePtr>, key: usize) {
        if let Some(node_ptr) = cell {
            assert_eq!(node_ptr.node().key, key);
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
        if let Some(node_ptr) = &bst.root {
            check_key(&node_ptr.node().left, 1);
        } else {
            panic!("BST must have root");
        }

        // New node becomes right node
        bst.put(3, "c".to_string());
        if let Some(node_ptr) = &bst.root {
            check_key(&node_ptr.node().right, 3);
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

    #[test]
    fn min() {
        let mut bst = BST::new();

        assert_eq!(bst.min(), None);

        populate_tree(&mut bst);

        assert_eq!(bst.min(), Some(1));
        bst.delete_min();
        assert_eq!(bst.min(), Some(2));
        bst.delete_min();
        assert_eq!(bst.min(), Some(3));
        bst.delete_min();
        assert_eq!(bst.min(), Some(4));
    }
}
