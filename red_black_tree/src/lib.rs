#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Color {
    Red,
    Black,
}

#[derive(Debug)]
pub struct Node<K, V> {
    key: K,
    value: V,
    left: Option<NodeId>,
    right: Option<NodeId>,
    color: Color,
    size: usize,
}

type NodeId = usize;

impl<K, V> Node<K, V> {
    pub fn new(key: K, value: V, color: Color) -> Self {
        Node {
            key,
            value,
            left: None,
            right: None,
            color,
            size: 1,
        }
    }
}

pub struct TreeIter<'a, K: 'a, V: 'a> {
    stack: Vec<NodeId>,
    tree: &'a RedBlackTree<K, V>,
}

impl<'a, K: 'a, V: 'a> TreeIter<'a, K, V> {
    fn push_left_edge(&mut self, mut node: Option<NodeId>) {
        while let Some(node_id) = node {
            self.stack.push(node_id);
            node = self.tree.nodes[node_id].left;
        }
    }
}

impl<'a, K, V> Iterator for TreeIter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<(&'a K, &'a V)> {
        let node_id = match self.stack.pop() {
            Some(n) => n,
            None => return None,
        };

        self.push_left_edge(self.tree.nodes[node_id].right);

        let node = &self.tree.nodes[node_id];
        Some((&node.key, &node.value))
    }
}

#[derive(Debug)]
pub struct RedBlackTree<K, V> {
    root: Option<NodeId>,
    nodes: Vec<Node<K, V>>,
}

impl<K, V> RedBlackTree<K, V>
    where K: PartialOrd + Clone,
          V: Clone,
{
    pub fn new() -> Self {
        RedBlackTree {
            root: None,
            nodes: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    pub fn size(&self) -> usize {
        self.size_of(self.root)
    }

    fn size_of(&self, node: Option<NodeId>) -> usize {
        node.map_or(0, |id| self.nodes[id].size)
    }

    fn update_size_for(&mut self, parent: NodeId) {
        let left_size = self.size_of(self.nodes[parent].left);
        let right_size = self.size_of(self.nodes[parent].right);
        self.nodes[parent].size = 1 + left_size + right_size;
    }

    fn new_node(&mut self, key: K, value: V, color: Color) -> NodeId {
        let next_index = self.nodes.len();

        self.nodes.push(Node::new(key, value, color));

        next_index
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let mut x = &self.root;
        while let Some(node_id) = x {
            let node = &self.nodes[*node_id];
            if *key < node.key {
                x = &node.left;
            } else if *key > node.key {
                x = &node.right;
            } else {
                return Some(&node.value);
            }
        }
        None
    }

    pub fn contains(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    pub fn put(&mut self, key: K, value: V) {
        self.root = self.rput(self.root, key, value);

        if let Some(root_id) = self.root {
            self.nodes[root_id].color = Color::Black;
        }
    }

    fn rput(&mut self, node: Option<NodeId>, key: K, value: V) -> Option<NodeId> {
        if let Some(node_id) = node {
            if key < self.nodes[node_id].key {
                self.nodes[node_id].left = self.rput(self.nodes[node_id].left, key, value);
            } else if key > self.nodes[node_id].key {
                self.nodes[node_id].right = self.rput(self.nodes[node_id].right, key, value);
            } else {
                self.nodes[node_id].value = value;
            }

            // Fix any right-leaning links
            let mut node_id = node_id;
            let right_is_red = self.is_red(self.nodes[node_id].right);
            let left_is_red = self.is_red(self.nodes[node_id].left);
            if right_is_red && !left_is_red {
                node_id = self.rotate_left(node_id);
            }
            if self.is_red(self.nodes[node_id].left) {
                let left_id = self.nodes[node_id].left.unwrap();
                if self.is_red(self.nodes[left_id].left) {
                    node_id = self.rotate_right(node_id);
                }
            }
            let left_is_red = self.is_red(self.nodes[node_id].left);
            let right_is_red = self.is_red(self.nodes[node_id].right);
            if left_is_red && right_is_red {
                self.flip_colors(node_id);
            }

            self.update_size_for(node_id);

            return Some(node_id);
        }

        let node_id = self.new_node(key, value, Color::Red);
        Some(node_id)
    }

    pub fn delete_min(&mut self) {
        if let Some(root_id) = self.root {
            let left_is_red = self.is_red(self.nodes[root_id].left);
            let right_is_red = self.is_red(self.nodes[root_id].right);
            if !left_is_red && !right_is_red {
                self.nodes[root_id].color = Color::Red;
            }

            self.root = self.rdelete_min(root_id);
            if let Some(root_id) = self.root {
                self.nodes[root_id].color = Color::Black;
            }
        }
    }

    fn rdelete_min(&mut self, node: NodeId) -> Option<NodeId> {
        let mut node_id = node;
        if self.nodes[node_id].left.is_none() {
            return None;
        }

        if !self.is_red(self.nodes[node_id].left) {
            let left_id = self.nodes[node_id].left.unwrap();
            if !self.is_red(self.nodes[left_id].left) {
                node_id = self.move_red_left(node_id);
            }
        }

        // Note: left can't be None, even with move_red_left operation
        let left_id = self.nodes[node_id].left.unwrap();
        self.nodes[node_id].left = self.rdelete_min(left_id);

        Some(self.balance(node_id))
    }

    pub fn delete(&mut self, key: &K) {
        if !self.contains(key) {
            return;
        }

        if let Some(root_id) = self.root {
            let left_is_red = self.is_red(self.nodes[root_id].left);
            let right_is_red = self.is_red(self.nodes[root_id].right);
            if !left_is_red && !right_is_red {
                self.nodes[root_id].color = Color::Red;
            }

            self.root = self.rdelete(root_id, key);
            if let Some(root_id) = self.root {
                self.nodes[root_id].color = Color::Black;
            }
        }
    }

    fn rdelete(&mut self, node: NodeId, key: &K) -> Option<NodeId> {
        let mut node_id = node;
        if *key < self.nodes[node_id].key {
            if !self.is_red(self.nodes[node_id].left) {
                let left_id = self.nodes[node_id].left.unwrap();
                if !self.is_red(self.nodes[left_id].left) {
                    node_id = self.move_red_left(node_id);
                }
            }
            let left_id = self.nodes[node_id].left.unwrap();
            self.nodes[node_id].left = self.rdelete(left_id, key);
        } else {
            if self.is_red(self.nodes[node_id].left) {
                node_id = self.rotate_right(node_id);
            }
            if *key == self.nodes[node_id].key && self.nodes[node_id].right.is_none() {
                return None;
            }
            
            if !self.is_red(self.nodes[node_id].right) {
                let right_id = self.nodes[node_id].right.unwrap();
                if !self.is_red(self.nodes[right_id].left) {
                    node_id = self.move_red_right(node_id);
                }
            }
            if *key == self.nodes[node_id].key {
                let right_id = self.nodes[node_id].right.unwrap();
                let x_id = self.rmin(right_id);
                self.nodes[node_id].key = self.nodes[x_id].key.clone();
                self.nodes[node_id].value = self.nodes[x_id].value.clone();
                self.nodes[node_id].right = self.rdelete_min(right_id);
            } else {
                let right_id = self.nodes[node_id].right.unwrap();
                self.nodes[node_id].right = self.rdelete(right_id, key);
            }
        }
        Some(self.balance(node_id))
    }


    // Red-black tree helper methods

    fn rotate_left(&mut self, parent: NodeId) -> NodeId {
        let old = parent;
        let new = self.nodes[old].right.unwrap();

        self.nodes[old].right = self.nodes[new].left;
        self.nodes[new].left = Some(old);
        self.nodes[new].color = self.nodes[old].color;
        self.nodes[old].color = Color::Red;
        self.nodes[new].size = self.nodes[old].size;
        self.update_size_for(old);

        new
    }

    fn rotate_right(&mut self, parent: NodeId) -> NodeId {
        let old = parent;
        let new = self.nodes[old].left.unwrap();

        self.nodes[old].left = self.nodes[new].right;
        self.nodes[new].right = Some(old);
        self.nodes[new].color = self.nodes[old].color;
        self.nodes[old].color = Color::Red;
        self.nodes[new].size = self.nodes[old].size;
        self.update_size_for(old);

        new
    }

    fn flip_colors(&mut self, node: NodeId) {
        let left = self.nodes[node].left.unwrap();
        let right = self.nodes[node].right.unwrap();
        match self.nodes[node].color {
            Color::Red => {
                self.nodes[node].color = Color::Black;
                self.nodes[left].color = Color::Red;
                self.nodes[right].color = Color::Red;
            },
            Color::Black => {
                self.nodes[node].color = Color::Red;
                self.nodes[left].color = Color::Black;
                self.nodes[right].color = Color::Black;
            },
        };
    }

    fn is_red(&self, node: Option<NodeId>) -> bool {
        match node {
            Some(node) => self.nodes[node].color == Color::Red,
            None => false,
        }
    }

    fn move_red_left(&mut self, node: NodeId) -> NodeId {
        let mut node_id = node;
        self.flip_colors(node_id);

        if let Some(right_id) = self.nodes[node_id].right {
            if self.is_red(self.nodes[right_id].left) {
                self.nodes[node_id].right = Some(self.rotate_right(right_id));
                node_id = self.rotate_left(node_id);
                self.flip_colors(node_id);
            }
        }

        node_id
    }

    fn move_red_right(&mut self, node: NodeId) -> NodeId {
        let mut node_id = node;
        self.flip_colors(node_id);

        if let Some(left_id) = self.nodes[node_id].left {
            if self.is_red(self.nodes[left_id].left) {
                node_id = self.rotate_right(node_id);
                self.flip_colors(node_id);
            }
        }

        node_id
    }

    fn balance(&mut self, node: NodeId) -> NodeId {
        let mut node_id = node;

        if self.is_red(self.nodes[node_id].right) {
            node_id = self.rotate_left(node_id);
        }

        if self.is_red(self.nodes[node_id].left) {
            let left_id = self.nodes[node_id].left.unwrap();
            if self.is_red(self.nodes[left_id].left) {
                node_id = self.rotate_right(node_id);
            }
        }

        let left_is_red = self.is_red(self.nodes[node_id].left);
        let right_is_red = self.is_red(self.nodes[node_id].right);
        if left_is_red && right_is_red {
            self.flip_colors(node_id);
        }

        self.update_size_for(node_id);
        node_id
    }

    // Ordered symbol table methods

    pub fn min(&self) -> Option<&K> {
        if let Some(root_id) = self.root {
            let min_id = self.rmin(root_id);
            return Some(&self.nodes[min_id].key);
        }
        None
    }

    fn rmin(&self, node: NodeId) -> NodeId {
        match self.nodes[node].left {
            Some(node_id) => self.rmin(node_id),
            None => node,
        }
    }

    pub fn max(&self) -> Option<&K> {
        if let Some(root_id) = self.root {
            let max_id = self.rmax(root_id);
            return Some(&self.nodes[max_id].key);
        }
        None
    }

    fn rmax(&self, node: NodeId) -> NodeId {
        match self.nodes[node].right {
            Some(node_id) => self.rmax(node_id),
            None => node,
        }
    }

    // Iterator
    pub fn iter(&self) -> TreeIter<K, V> {
        let mut iter = TreeIter {
            stack: Vec::new(),
            tree: self,
        };
        iter.push_left_edge(self.root);
        iter
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //      E
    //     / \
    //    C   R
    //   /   / \
    //  A   H   S
    fn populate_tree(tree: &mut RedBlackTree<String, usize>) {
        tree.put("S".to_string(), 0);
        tree.put("E".to_string(), 12);
        tree.put("A".to_string(), 8);
        tree.put("R".to_string(), 3);
        tree.put("C".to_string(), 4);
        tree.put("H".to_string(), 5);
    }

    #[test]
    fn create_node() {
        let n = Node::new(1, "a".to_string(), Color::Red);
        assert_eq!(n.left.is_none(), true);
        assert_eq!(n.right.is_none(), true);
    }

    #[test]
    fn get_value() {
        let mut tree = RedBlackTree::new();

        // empty tree case
        let s = "S".to_string();
        assert_eq!(tree.contains(&s), false);
        assert_eq!(tree.get(&s), None);

        populate_tree(&mut tree);

        assert_eq!(tree.contains(&s), true);
        assert_eq!(tree.get(&s), Some(&0));
        assert_eq!(tree.get(&"H".to_string()), Some(&5));
        assert_eq!(tree.contains(&"Z".to_string()), false);
    }

    #[test]
    fn put() {
        let mut tree = RedBlackTree::new();

        tree.put("E".to_string(), 12);

        // check root is black
        let root_id = tree.root.unwrap();
        let root = &tree.nodes[root_id];
        assert_eq!(root.color, Color::Black);

        //   E           S
        //    \   ->    /
        //     S       E
        tree.put("S".to_string(), 0);

        // check left-rotate of right-leaning link
        let root_id = tree.root.unwrap();
        let root = &tree.nodes[root_id];
        let left_id = root.left.unwrap();
        let left = &tree.nodes[left_id];
        assert_eq!(left.key, "E".to_string());
        assert_eq!(left.color, Color::Red);

        //      S
        //     /          E
        //    E    ->    / \
        //   /          A   S
        //  A
        tree.put("A".to_string(), 8);

        // check right-rotate then color-flip
        let root_id = tree.root.unwrap();
        let root = &tree.nodes[root_id];
        assert_eq!(root.key, "E".to_string());
        assert_eq!(root.color, Color::Black);

        let left_id = root.left.unwrap();
        let left = &tree.nodes[left_id];
        assert_eq!(left.key, "A".to_string());
        assert_eq!(left.color, Color::Black);

        let right_id = root.right.unwrap();
        let right = &tree.nodes[right_id];
        assert_eq!(right.key, "S".to_string());
        assert_eq!(right.color, Color::Black);
    }

    #[test]
    fn delete_min() {
        // Case: Empty RBT
        let mut tree0: RedBlackTree<String, usize> = RedBlackTree::new();
        tree0.delete_min();

        // Case: Delete from 2-node and 1-node tree
        let mut tree2 = RedBlackTree::new();

        //    S
        //   /
        //  E
        //
        let e = "E".to_string();
        let s = "S".to_string();
        tree2.put(s.clone(), 1);
        tree2.put(e.clone(), 2);
        tree2.delete_min();
        assert_eq!(tree2.get(&e), None);
        assert_eq!(tree2.min(), Some(&s));
        let root_id = tree2.root.unwrap();
        let root = &tree2.nodes[root_id];
        assert_eq!(root.color, Color::Black);
        tree2.delete_min();
        assert_eq!(tree2.root, None);

        // Case: balanced 7-node tree with black links
        let mut tree7 = RedBlackTree::new();

        //        L
        //      /   \
        //     H     T
        //    / \   / \
        //   D   J P   X
        //
        tree7.put("L".to_string(), 1);
        tree7.put("H".to_string(), 2);
        tree7.put("T".to_string(), 3);
        tree7.put("P".to_string(), 4);
        tree7.put("X".to_string(), 5);
        tree7.put("D".to_string(), 6);
        tree7.put("J".to_string(), 7);
        tree7.delete_min();

        //        T
        //      //  \
        //      L    X
        //     / \
        //    J   P     Legend:  / - black link
        //   //                 // - red link
        //   H
        //
        let root_id = tree7.root.unwrap();
        let root = &tree7.nodes[root_id];
        assert_eq!(root.key, "T".to_string());
        let left_id = root.left.unwrap();
        let left = &tree7.nodes[left_id];
        assert_eq!(left.key, "L".to_string());
        assert_eq!(left.color, Color::Red);
        let ll_id = left.left.unwrap();
        let ll = &tree7.nodes[ll_id];
        assert_eq!(ll.key, "J".to_string());
        assert_eq!(ll.color, Color::Black);
        let lr_id = left.right.unwrap();
        let lr = &tree7.nodes[lr_id];
        assert_eq!(lr.key, "P".to_string());
        assert_eq!(lr.color, Color::Black);
        let lll_id = ll.left.unwrap();
        let lll = &tree7.nodes[lll_id];
        assert_eq!(lll.key, "H".to_string());
        assert_eq!(lll.color, Color::Red);
    }

    #[test]
    fn delete() {
        let s = "S".to_string();
        let e = "E".to_string();

        // Case: Empty RBT
        let mut tree0: RedBlackTree<String, usize> = RedBlackTree::new();
        tree0.delete(&s);

        // Case: Delete from 2-node and 1-node tree
        let mut tree2 = RedBlackTree::new();

        //    S
        //   /
        //  E
        //
        tree2.put("S".to_string(), 1);
        tree2.put("E".to_string(), 2);
        tree2.delete(&e);
        assert_eq!(tree2.get(&e), None);
        let root_id = tree2.root.unwrap();
        let root = &tree2.nodes[root_id];
        assert_eq!(root.color, Color::Black);
        tree2.delete(&s);
        assert_eq!(tree2.root, None);

        // Case: 4-node tree
        let mut tree4 = RedBlackTree::new();

        //    E           R
        //   / \         / \
        //  A   S   ->  A   S
        //     //
        //     R
        //
        tree4.put("S".to_string(), 1);
        tree4.put("E".to_string(), 2);
        tree4.put("A".to_string(), 3);
        tree4.put("R".to_string(), 4);
        tree4.delete(&e);
        assert_eq!(tree4.get(&e), None);
        let root_id = tree4.root.unwrap();
        let root = &tree4.nodes[root_id];
        assert_eq!(root.key, "R".to_string());
        let right_id = root.right.unwrap();
        let right = &tree4.nodes[right_id];
        assert_eq!(right.key, "S".to_string());
        assert_eq!(right.color, Color::Black);
    }

    #[test]
    fn tree_size() {
        let mut tree = RedBlackTree::new();

        assert_eq!(tree.is_empty(), true);
        assert_eq!(tree.size(), 0);

        populate_tree(&mut tree);

        assert_eq!(tree.is_empty(), false);
        assert_eq!(tree.size(), 6);
    }

    #[test]
    fn min() {
        let mut tree = RedBlackTree::new();

        assert_eq!(tree.min(), None);

        populate_tree(&mut tree);

        assert_eq!(tree.min(), Some(&"A".to_string()));
    }

    #[test]
    fn max() {
        let mut tree = RedBlackTree::new();

        assert_eq!(tree.max(), None);

        populate_tree(&mut tree);

        assert_eq!(tree.max(), Some(&"S".to_string()));
    }

    #[test]
    fn iterator() {
        let mut tree = RedBlackTree::new();
        populate_tree(&mut tree);
        let mut tree_iter = tree.iter();

        assert_eq!(tree_iter.next(), Some((&"A".to_string(), &8)));
        assert_eq!(tree_iter.next(), Some((&"C".to_string(), &4)));
        assert_eq!(tree_iter.next(), Some((&"E".to_string(), &12)));
        assert_eq!(tree_iter.next(), Some((&"H".to_string(), &5)));
        assert_eq!(tree_iter.next(), Some((&"R".to_string(), &3)));
        assert_eq!(tree_iter.next(), Some((&"S".to_string(), &0)));
        assert_eq!(tree_iter.next(), None);
    }
}
