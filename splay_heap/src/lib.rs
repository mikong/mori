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
    pub fn insert(self, element: T) -> Tree<T> {
        let (smaller, bigger) = self.partition(&element);
        Tree::NonEmpty(Box::new(Node {
            element: element,
            left: smaller,
            right: bigger,
        }))
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
        let _tree = NonEmpty(Box::new(Node {
            element: 5,
            left: Empty,
            right: Empty,
        }));
    }
}
