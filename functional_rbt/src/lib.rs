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
    pub fn new(color: Color, element: T, left: Tree<T>, right: Tree<T>) -> Tree<T> {
        Tree::NonEmpty(Box::new(Node {
            color,
            element,
            left,
            right,
        }))
    }

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

    pub fn insert(self, element: T) -> Tree<T> {
        match self {
            Tree::Empty => {
                Tree::new(
                    Color::Red,
                    element,
                    Tree::Empty,
                    Tree::Empty
                )
            },
            Tree::NonEmpty(node) => {
                if element < node.element {
                    Tree::new(
                        Color::Black,
                        node.element,
                        node.left.insert(element),
                        node.right
                    )
                } else if element > node.element {
                    Tree::new(
                        Color::Black,
                        node.element,
                        node.left,
                        node.right.insert(element)
                    )
                } else {
                    Tree::NonEmpty(node)
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
        let tree = Tree::new(Red, 5, Empty, Empty);

        assert_eq!(tree.member(&5), true);
        assert_eq!(tree.member(&6), false);
    }
}
