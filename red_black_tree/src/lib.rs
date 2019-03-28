#[derive(Debug)]
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
    left: Option<Box<Node<K, V>>>,
    right: Option<Box<Node<K, V>>>,
    color: Color,
}

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
    root: Option<Box<Node<K, V>>>,
}

impl<K, V> RedBlackTree<K, V>
    where K: PartialOrd
{
    pub fn new() -> Self {
        RedBlackTree { root: None }
    }

    pub fn get(&self, key: K) -> Option<&V> {
        RedBlackTree::get_value(&self.root, key)
    }

    fn get_value(x: &Option<Box<Node<K, V>>>, key: K) -> Option<&V> {
        let mut x = x;
        while let Some(node) = x {
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
