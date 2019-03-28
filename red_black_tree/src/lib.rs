#[derive(Debug, Copy, Clone)]
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

    // Helper methods

    fn rotate_left(&mut self, parent: &Option<NodeId>) -> Option<NodeId> {
        let old = parent.unwrap();
        let new = self.nodes[old].right.unwrap();

        self.nodes[old].right = self.nodes[new].left;
        self.nodes[new].left = Some(old);
        self.nodes[new].color = self.nodes[old].color;
        self.nodes[old].color = Color::Red;

        Some(new)
    }

    fn rotate_right(&mut self, parent: &Option<NodeId>) -> Option<NodeId> {
        let old = parent.unwrap();
        let new = self.nodes[old].left.unwrap();

        self.nodes[old].left = self.nodes[new].right;
        self.nodes[new].right = Some(old);
        self.nodes[new].color = self.nodes[old].color;
        self.nodes[old].color = Color::Red;

        Some(new)
    }

    fn flip_colors(&mut self, node: &Option<NodeId>) {
        let node = node.unwrap();
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
}
