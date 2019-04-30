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

impl<T> Tree<T> {
    pub fn new(element: T, left: Tree<T>, right: Tree<T>) -> Tree<T> {
        Tree::NonEmpty(Box::new(Node {
            element,
            left,
            right,
        }))
    }

    pub fn insert(self, element: T) -> Tree<T> {
        let (smaller, bigger) = self.partition(&element);
        Tree::new(element, smaller, bigger)
    }

    fn partition(self, element: &T) -> (Tree<T>, Tree<T>) {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::Tree::*;

    #[test]
    fn it_works() {
        let _tree = Tree::new(5, Empty, Empty);
    }
}
