use std::rc::Rc;
use std::cell::RefCell;
use std::cell::Ref;

#[derive(Debug)]
pub struct Node(Rc<RefCell<RawNode>>);

impl Node {
    pub fn new(key: usize, value: String) -> Self {
        let node = Rc::new(RefCell::new(RawNode {
            key,
            value,
            left: None,
            right: None,
            size: 1,
        }));
        Node(node)
    }

    pub fn get(&self) -> Ref<RawNode> {
        self.0.borrow()
    }

    pub fn set_value(&self, value: String) {
        self.0.borrow_mut().value = value;
    }

    pub fn set_left(&self, node: Option<Node>) {
        self.0.borrow_mut().left = node;
    }

    pub fn set_right(&self, node: Option<Node>) {
        self.0.borrow_mut().right = node;
    }

    pub fn update_size(&self) {
        let size = 1 + Node::size(&self.get().left) + Node::size(&self.get().right);
        self.0.borrow_mut().size = size;
    }

    pub fn clone(&self) -> Self {
        Node(Rc::clone(&self.0))
    }

    pub fn size(node: &Option<Node>) -> usize {
        node.as_ref().map_or(0, |n| n.0.borrow().size)
    }
}

#[derive(Debug)]
pub struct RawNode {
    key: usize,
    value: String,
    left: Option<Node>,
    right: Option<Node>,
    size: usize,
}

#[derive(Debug)]
pub struct BST {
    root: Option<Node>,
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

    fn getr(x: &Option<Node>, key: usize) -> Option<String> {
        if let Some(node) = x {
            if key < node.get().key {
                return BST::getr(&node.get().left, key);
            } else if key > node.get().key {
                return BST::getr(&node.get().right, key);
            } else {
                return Some(node.get().value.clone());
            }
        }
        None
    }

    pub fn put(&mut self, key: usize, value: String) {
        self.root = BST::upsert(&self.root, key, value);
    }

    fn upsert(x: &Option<Node>, key: usize, value: String) -> Option<Node> {
        if let Some(node) = x {
            if key < node.get().key {
                let new_node = BST::upsert(&node.get().left, key, value);
                node.set_left(new_node);
            } else if key > node.get().key {
                let new_node = BST::upsert(&node.get().right, key, value);
                node.set_right(new_node);
            } else {
                node.set_value(value);
            }
            node.update_size();
            return Some(node.clone());
        }
        // x = None
        Some(Node::new(key, value))
    }

    pub fn delete_min(&mut self) {
        self.root = BST::remove_min(&self.root);
    }

    fn remove_min(x: &Option<Node>) -> Option<Node> {
        if let Some(node) = x {
            if node.get().left.is_none() {
                return node.get().right.as_ref().map(|n| n.clone());
            }
            let new_node = BST::remove_min(&node.get().left);
            node.set_left(new_node);
            node.update_size();
            return Some(node.clone());
        }
        None
    }

    pub fn delete(&mut self, key: usize) {
        self.root = BST::remove(&self.root, key);
    }

    fn remove(x: &Option<Node>, key: usize) -> Option<Node> {
        if let Some(node) = x {
            if key < node.get().key {
                let new_node = BST::remove(&node.get().left, key);
                node.set_left(new_node);
            } else if key > node.get().key {
                let new_node = BST::remove(&node.get().right, key);
                node.set_right(new_node);
            } else {
                if node.get().right.is_none() {
                    return node.get().left.as_ref().map(|n| n.clone());
                }
                if node.get().left.is_none() {
                    return node.get().right.as_ref().map(|n| n.clone());
                }

                let temp = node.clone();
                // minimum of the right replaces node to be deleted
                let node = BST::minimum(&temp.get().right).unwrap();

                // new node takes left and right of the deleted
                let right = BST::remove_min(&temp.get().right);
                node.set_right(right);
                let left = temp.get().left.as_ref().map(|n| n.clone());
                node.set_left(left);

                node.update_size();
                return Some(node);
            }
            node.update_size();
            return Some(node.clone());
        }

        None
    }

    pub fn min(&self) -> Option<usize> {
        if let Some(node) = BST::minimum(&self.root) {
            return Some(node.get().key)
        }

        None
    }

    fn minimum(x: &Option<Node>) -> Option<Node> {
        if let Some(node) = x {
            if node.get().left.is_none() {
                return Some(node.clone());
            } else {
                return BST::minimum(&node.get().left);
            }
        }

        None
    }

    pub fn max(&self) -> Option<usize> {
        if let Some(node) = BST::maximum(&self.root) {
            return Some(node.get().key)
        }

        None
    }

    fn maximum(x: &Option<Node>) -> Option<Node> {
        if let Some(node) = x {
            if node.get().right.is_none() {
                return Some(node.clone());
            } else {
                return BST::maximum(&node.get().right);
            }
        }

        None
    }

    pub fn keys(&self) -> Vec<usize> {
        let mut v = Vec::new();
        BST::inorder(&self.root, &mut v);
        v
    }

    fn inorder(x: &Option<Node>, v: &mut Vec<usize>) {
        if let Some(node) = x {
            BST::inorder(&node.get().left, v);
            v.push(node.get().key);
            BST::inorder(&node.get().right, v);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_key(cell: &Option<Node>, key: usize) {
        if let Some(node) = cell {
            assert_eq!(node.get().key, key);
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
        assert_eq!(n.get().left.is_none(), true);
        assert_eq!(n.get().right.is_none(), true);
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
            check_key(&node.get().left, 1);
        } else {
            panic!("BST must have root");
        }

        // New node becomes right node
        bst.put(3, "c".to_string());
        if let Some(node) = &bst.root {
            check_key(&node.get().right, 3);
        } else {
            panic!("BST must have root");
        }
    }

    #[test]
    fn update() {
        let mut bst = BST::new();
        populate_tree(&mut bst);

        assert_eq!(bst.get(5), Some("H".to_string()));
        bst.put(5, "I".to_string());
        assert_eq!(bst.get(5), Some("I".to_string()));
        assert_eq!(bst.size(), 9);
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

    #[test]
    fn max() {
        let mut bst = BST::new();

        assert_eq!(bst.max(), None);

        populate_tree(&mut bst);

        assert_eq!(bst.max(), Some(9));
    }

    #[test]
    fn delete() {
        let mut bst = BST::new();

        // delete any key of an empty BST
        bst.delete(8);

        populate_tree(&mut bst);
        assert_eq!(bst.size(), 9);

        bst.delete(3);
        assert_eq!(bst.size(), 8);
        assert_eq!(bst.keys(), vec![1, 2, 4, 5, 6, 7, 8, 9]);
        if let Some(node) = &bst.root {
            check_key(&node.get().left, 4);
        }

        bst.delete(7);
        assert_eq!(bst.size(), 7);
        assert_eq!(bst.keys(), vec![1, 2, 4, 5, 6, 8, 9]);
    }
}
