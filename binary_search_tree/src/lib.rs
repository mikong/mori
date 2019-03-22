#[derive(Debug)]
pub struct Node {
    key: usize,
    value: String,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let n = Node::new(1, "a".to_string());
        assert_eq!(n.left.is_none(), true);
        assert_eq!(n.right.is_none(), true);
    }
}
