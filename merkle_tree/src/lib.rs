#[derive(Debug)]
pub enum MerkleTree<T> {
    Empty,
    NonEmpty(Box<Node<T>>),
}

#[derive(Debug)]
pub struct Node<T> {
    element: T,
    left: MerkleTree<T>,
    right: MerkleTree<T>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::MerkleTree::*;

    #[test]
    fn it_works() {
        let _tree = NonEmpty(Box::new(Node {
            element: 5,
            left: Empty,
            right: Empty,
        }));
    }
}
