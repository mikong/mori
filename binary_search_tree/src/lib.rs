use std::rc::Rc;
use std::cell::RefCell;
use std::cell::Ref;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct Node<V: Clone>(Rc<RefCell<RawNode<V>>>);

impl<V: Clone> Node<V> {
    pub fn new(key: usize, value: V) -> Self {
        let node = Rc::new(RefCell::new(RawNode {
            key,
            value,
            left: None,
            right: None,
            size: 1,
        }));
        Node(node)
    }

    pub fn get(&self) -> Ref<RawNode<V>> {
        self.0.borrow()
    }

    pub fn set_value(&self, value: V) {
        self.0.borrow_mut().value = value;
    }

    pub fn set_left(&self, node: Option<Node<V>>) {
        self.0.borrow_mut().left = node;
    }

    pub fn set_right(&self, node: Option<Node<V>>) {
        self.0.borrow_mut().right = node;
    }

    pub fn update_size(&self) {
        let size = 1 + Node::size(&self.get().left) + Node::size(&self.get().right);
        self.0.borrow_mut().size = size;
    }

    pub fn clone(&self) -> Self {
        Node(Rc::clone(&self.0))
    }

    pub fn size(node: &Option<Node<V>>) -> usize {
        node.as_ref().map_or(0, |n| n.0.borrow().size)
    }
}

#[derive(Debug)]
pub struct RawNode<V: Clone> {
    key: usize,
    value: V,
    left: Option<Node<V>>,
    right: Option<Node<V>>,
    size: usize,
}

pub struct TreeIter<V: Clone> {
    unvisited: Vec<Node<V>>,
}

impl<V: Clone> TreeIter<V> {
    fn push_left_edge(&mut self, x: &Option<Node<V>>) {
        if let Some(ref node) = *x {
            self.unvisited.push(node.clone());
            self.push_left_edge(&node.get().left);
        }
    }
}

impl<V: Clone> Iterator for TreeIter<V> {
    type Item = (usize, V);

    fn next(&mut self) -> Option<(usize, V)> {
        let node = match self.unvisited.pop() {
            Some(n) => n,
            None => return None,
        };

        self.push_left_edge(&node.get().right);

        let key = node.get().key;
        let value = node.get().value.clone();
        Some((key, value))
    }
}

#[derive(Debug)]
pub struct BST<V: Clone> {
    root: Option<Node<V>>,
}

impl<V: Clone> BST<V> {
    /// Creates a new empty Binary Search Tree.
    pub fn new() -> Self {
        BST { root: None }
    }

    /// Returns `true` if the tree has no node elements.
    pub fn is_empty(&self) -> bool {
        Node::size(&self.root) == 0
    }

    /// Returns the number of elements in the tree.
    pub fn size(&self) -> usize {
        Node::size(&self.root)
    }

    pub fn contains(&self, key: usize) -> bool {
        self.get(key).is_some()
    }

    /// Returns a clone of the value associated with the given key.
    pub fn get(&self, key: usize) -> Option<V> {
        BST::get_value(&self.root, key)
    }

    fn get_value(x: &Option<Node<V>>, key: usize) -> Option<V> {
        if let Some(node) = x {
            if key < node.get().key {
                return BST::get_value(&node.get().left, key);
            } else if key > node.get().key {
                return BST::get_value(&node.get().right, key);
            } else {
                return Some(node.get().value.clone());
            }
        }
        None
    }

    /// Inserts the given key-value pair into the tree. If the tree already
    /// contains the given key, the associated value is updated.
    pub fn put(&mut self, key: usize, value: V) {
        self.root = BST::upsert(&self.root, key, value);
    }

    fn upsert(x: &Option<Node<V>>, key: usize, value: V) -> Option<Node<V>> {
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

    /// Removes the smallest key and its associated value from the tree.
    pub fn delete_min(&mut self) {
        self.root = BST::remove_min(&self.root);
    }

    fn remove_min(x: &Option<Node<V>>) -> Option<Node<V>> {
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

    /// Removes the largest key and its associated value from the tree.
    pub fn delete_max(&mut self) {
        self.root = BST::remove_max(&self.root);
    }

    fn remove_max(x: &Option<Node<V>>) -> Option<Node<V>> {
        if let Some(node) = x {
            if node.get().right.is_none() {
                return node.get().left.as_ref().map(|n| n.clone());
            }
            let new_node = BST::remove_max(&node.get().right);
            node.set_right(new_node);
            node.update_size();
            return Some(node.clone());
        }
        None
    }

    /// Removes the given key and its associated value from the tree.
    pub fn delete(&mut self, key: usize) {
        self.root = BST::remove(&self.root, key);
    }

    fn remove(x: &Option<Node<V>>, key: usize) -> Option<Node<V>> {
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

    /// Returns the smallest key in the tree.
    pub fn min(&self) -> Option<usize> {
        if let Some(node) = BST::minimum(&self.root) {
            return Some(node.get().key)
        }

        None
    }

    fn minimum(x: &Option<Node<V>>) -> Option<Node<V>> {
        if let Some(node) = x {
            if node.get().left.is_none() {
                return Some(node.clone());
            } else {
                return BST::minimum(&node.get().left);
            }
        }

        None
    }

    /// Returns the largest key in the tree.
    pub fn max(&self) -> Option<usize> {
        if let Some(node) = BST::maximum(&self.root) {
            return Some(node.get().key)
        }

        None
    }

    fn maximum(x: &Option<Node<V>>) -> Option<Node<V>> {
        if let Some(node) = x {
            if node.get().right.is_none() {
                return Some(node.clone());
            } else {
                return BST::maximum(&node.get().right);
            }
        }

        None
    }

    pub fn iter(&self) -> TreeIter<V> {
        let mut iter = TreeIter { unvisited: Vec::new() };
        iter.push_left_edge(&self.root);
        iter
    }

    pub fn keys(&self) -> Vec<usize> {
        let mut v = Vec::new();
        BST::inorder(&self.root, &mut v);
        v
    }

    fn inorder(x: &Option<Node<V>>, v: &mut Vec<usize>) {
        if let Some(node) = x {
            BST::inorder(&node.get().left, v);
            v.push(node.get().key);
            BST::inorder(&node.get().right, v);
        }
    }

    pub fn level_order(&self) -> Vec<usize> {
        let mut keys = Vec::new();
        let mut queue = VecDeque::new();
        if let Some(node) = &self.root {
            queue.push_back(node.clone());
        }
        while !queue.is_empty() {
            let node = queue.pop_front().unwrap();
            keys.push(node.get().key);
            if let Some(n) = &node.get().left {
                queue.push_back(n.clone());
            };
            if let Some(n) = &node.get().right {
                queue.push_back(n.clone());
            };
        }
        keys
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_key(cell: &Option<Node<String>>, key: usize) {
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
    fn populate_tree(bst: &mut BST<String>) {
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

        // empty tree case
        assert_eq!(bst.contains(8), false);
        assert_eq!(bst.get(8), None);

        populate_tree(&mut bst);

        assert_eq!(bst.contains(8), true);
        assert_eq!(bst.get(8), Some("S".to_string()));
        assert_eq!(bst.get(2), Some("C".to_string()));
        assert_eq!(bst.get(9), Some("X".to_string()));
        assert_eq!(bst.get(5), Some("H".to_string()));

        // key not in tree
        assert_eq!(bst.contains(10), false);
        assert_eq!(bst.get(10), None);

        // after delete
        bst.delete(8);
        assert_eq!(bst.contains(8), false);
        assert_eq!(bst.get(8), None);
    }

    #[test]
    fn inorder_traversal() {
        let mut bst = BST::new();
        populate_tree(&mut bst);

        assert_eq!(bst.keys().len(), 9);
        assert_eq!(bst.keys(), vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn levelorder_traversal() {
        let mut bst = BST::new();
        populate_tree(&mut bst);

        assert_eq!(bst.level_order().len(), 9);
        assert_eq!(bst.level_order(), vec![8, 3, 9, 1, 7, 2, 5, 4, 6]);
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
    fn delete_min() {
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
    fn delete_max() {
        let mut bst = BST::new();

        // delete max an empty BST
        bst.delete_max();

        populate_tree(&mut bst);

        bst.delete_max();
        assert_eq!(bst.keys(), vec![1, 2, 3, 4, 5, 6, 7, 8]);
        bst.delete_max();
        assert_eq!(bst.keys(), vec![1, 2, 3, 4, 5, 6, 7]);
        bst.delete_max();
        assert_eq!(bst.keys(), vec![1, 2, 3, 4, 5, 6]);
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

    #[test]
    fn iterator() {
        let mut bst = BST::new();
        populate_tree(&mut bst);
        let mut tree_iter = bst.iter();

        assert_eq!(tree_iter.next(), Some((1, "A".to_string())));
        assert_eq!(tree_iter.next(), Some((2, "C".to_string())));
        assert_eq!(tree_iter.next(), Some((3, "E".to_string())));
    }
}
