#[derive(Debug)]
pub enum Color {
    Red,
    Black,
}

#[derive(Debug)]
pub enum Tree<T> {
    Empty,
    NonEmpty(Box<Node<T>>),
}

#[derive(Debug)]
pub struct Node<T> {
    color: Color,
    element: T,
    left: Tree<T>,
    right: Tree<T>,
}

impl<T: Ord> Tree<T> {
    pub fn member(&self, element: &T) -> bool {
        match self {
            Tree::Empty => false,
            Tree::NonEmpty(ref node) => {
                if *element < node.element {
                    node.left.member(element)
                } else if *element > node.element {
                    node.right.member(element)
                } else {
                    true
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::Color::*;
    use super::Tree::*;

    #[test]
    fn it_works() {
        let tree = NonEmpty(Box::new(Node {
            color: Red,
            element: 5,
            left: Empty,
            right: Empty,
        }));

        assert_eq!(tree.member(&5), true);
        assert_eq!(tree.member(&6), false);
    }
}
