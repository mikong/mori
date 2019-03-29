#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Color {
    Red,
    Black,
}

#[derive(Debug)]
pub struct Node<K, V>
    where K: PartialOrd
{
    key: K,
    value: V,
    left: Option<NodeId>,
    right: Option<NodeId>,
    color: Color,
}

type NodeId = usize;

impl<K, V> Node<K, V>
    where K: PartialOrd
{
    pub fn new(key: K, value: V, color: Color) -> Self {
        Node {
            key,
            value,
            left: None,
            right: None,
            color,
        }
    }
}

#[derive(Debug)]
pub struct RedBlackTree<K, V>
    where K: PartialOrd
{
    root: Option<NodeId>,
    nodes: Vec<Node<K, V>>,
}

impl<K, V> RedBlackTree<K, V>
    where K: PartialOrd
{
    pub fn new() -> Self {
        RedBlackTree {
            root: None,
            nodes: Vec::new(),
        }
    }

    fn new_node(&mut self, key: K, value: V, color: Color) -> NodeId {
        let next_index = self.nodes.len();

        self.nodes.push(Node::new(key, value, color));

        next_index
    }

    pub fn get(&self, key: K) -> Option<&V> {
        let mut x = &self.root;
        while let Some(node_id) = x {
            let node = &self.nodes[*node_id];
            if key < node.key {
                x = &node.left;
            } else if key > node.key {
                x = &node.right;
            } else {
                return Some(&node.value);
            }
        }
        None
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

            return Some(node_id);
        }

        let node_id = self.new_node(key, value, Color::Red);
        Some(node_id)
    }

    // Helper methods

    fn rotate_left(&mut self, parent: NodeId) -> NodeId {
        let old = parent;
        let new = self.nodes[old].right.unwrap();

        self.nodes[old].right = self.nodes[new].left;
        self.nodes[new].left = Some(old);
        self.nodes[new].color = self.nodes[old].color;
        self.nodes[old].color = Color::Red;

        new
    }

    fn rotate_right(&mut self, parent: NodeId) -> NodeId {
        let old = parent;
        let new = self.nodes[old].left.unwrap();

        self.nodes[old].left = self.nodes[new].right;
        self.nodes[new].right = Some(old);
        self.nodes[new].color = self.nodes[old].color;
        self.nodes[old].color = Color::Red;

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_node() {
        let n = Node::new(1, "a".to_string(), Color::Red);
        assert_eq!(n.left.is_none(), true);
        assert_eq!(n.right.is_none(), true);
    }

    #[test]
    fn get_value() {
        let tree: RedBlackTree<usize, String> = RedBlackTree::new();

        // empty tree case
        assert_eq!(tree.get(8), None);

        // TODO: non-empty case
    }

    #[test]
    fn put() {
        let mut tree = RedBlackTree::new();
        tree.put("A".to_string(), 8);
        tree.put("C".to_string(), 4);
        tree.put("E".to_string(), 12);

        let root_id = tree.root.unwrap();
        let root = &tree.nodes[root_id];
        assert_eq!(root.key, "C".to_string());
        assert_eq!(root.color, Color::Black);

        let left_id = root.left.unwrap();
        let left = &tree.nodes[left_id];
        assert_eq!(left.key, "A".to_string());
        assert_eq!(left.color, Color::Black);

        let right_id = root.right.unwrap();
        let right = &tree.nodes[right_id];
        assert_eq!(right.key, "E".to_string());
        assert_eq!(right.color, Color::Black);
    }
}
