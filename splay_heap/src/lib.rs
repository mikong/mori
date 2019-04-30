#[derive(Debug)]
pub enum Tree<T> {
    Empty,
    NonEmpty(Box<Node<T>>),
}

#[derive(Debug)]
pub struct Node<T> {
    element: T,
    left: Tree<T>,
    right: Tree<T>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::Tree::*;

    #[test]
    fn it_works() {
        let _tree = NonEmpty(Box::new(Node {
            element: 5,
            left: Empty,
            right: Empty,
        }));
    }
}
