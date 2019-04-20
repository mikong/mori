#[derive(Debug, Copy, Clone, PartialEq)]
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

    fn is_red(&self) -> bool {
        match self {
            Tree::NonEmpty(node) => node.color == Color::Red,
            Tree::Empty => false,
        }
    }

    fn left_is_red(&self) -> bool {
        match self {
            Tree::NonEmpty(node) => node.left.is_red(),
            Tree::Empty => false,
        }
    }

    fn right_is_red(&self) -> bool {
        match self {
            Tree::NonEmpty(node) => node.right.is_red(),
            Tree::Empty => false,
        }
    }

    //                   z
    //                  / \
    //                (x)  d
    //                / \
    //               a  (y)
    //                  / \
    //                 b   c
    //
    //                   ⬇︎
    //       z                       x
    //      / \         (y)         / \
    //    (y)  d  ->   /   \   <-  a  (y)
    //    / \         x     z         / \
    //  (x)  c       / \   / \       b  (z)
    //  / \         a  b  c   d         / \
    // a   b                           c   d
    //                   ⬆︎
    //
    //                  x
    //                 / \
    //                a  (z)
    //                   / \
    //                 (y)   d
    //                 / \
    //                b   c
    //
    fn balance(color: Color, element: T, left: Tree<T>, right: Tree<T>) -> Tree<T> {
        if color == Color::Black && left.is_red() && left.left_is_red() {
            if let Tree::NonEmpty(node) = left {
                if let Tree::NonEmpty(lnode) = node.left {
                    let new_l = Tree::new(Color::Black, lnode.element, lnode.left, lnode.right);
                    let new_r = Tree::new(Color::Black, element, node.right, right);
                    return Tree::new(Color::Red, node.element, new_l, new_r);
                }
            }
        } else if color == Color::Black && left.is_red() && left.right_is_red() {
            if let Tree::NonEmpty(node) = left {
                if let Tree::NonEmpty(rnode) = node.right {
                    let new_l = Tree::new(Color::Black, node.element, node.left, rnode.left);
                    let new_r = Tree::new(Color::Black, element, rnode.right, right);
                    return Tree::new(Color::Red, rnode.element, new_l, new_r);
                }
            }
        } else if color == Color::Black && right.is_red() && right.left_is_red() {
            if let Tree::NonEmpty(node) = right {
                if let Tree::NonEmpty(lnode) = node.left {
                    let new_l = Tree::new(Color::Black, element, left, lnode.left);
                    let new_r = Tree::new(Color::Black, node.element, lnode.right, node.right);
                    return Tree::new(Color::Red, lnode.element, new_l, new_r);
                }
            }
        } else if color == Color::Black && right.is_red() && right.right_is_red() {
            if let Tree::NonEmpty(node) = right {
                if let Tree::NonEmpty(rnode) = node.right {
                    let new_l = Tree::new(Color::Black, element, left, node.left);
                    let new_r = Tree::new(Color::Black, rnode.element, rnode.left, rnode.right);
                    return Tree::new(Color::Red, node.element, new_l, new_r);
                }
            }
        } else {
            return Tree::new(color, element, left, right);
        }

        unreachable!();
    }

    fn ins(self, element: T) -> Tree<T> {
        match self {
            Tree::Empty => {
                Tree::new(Color::Red, element, Tree::Empty, Tree::Empty)
            },
            Tree::NonEmpty(node) => {
                if element < node.element {
                    Tree::balance(
                        node.color,
                        node.element,
                        node.left.ins(element),
                        node.right,
                    )
                } else if element > node.element {
                    Tree::balance(
                        node.color,
                        node.element,
                        node.left,
                        node.right.ins(element),
                    )
                } else {
                    Tree::NonEmpty(node)
                }
            },
        }
    }

    pub fn insert(self, element: T) -> Tree<T> {
        let tree = self.ins(element);
        match tree {
            Tree::NonEmpty(node) => {
                Tree::new(
                    Color::Black,
                    node.element,
                    node.left,
                    node.right,
                )
            },
            Tree::Empty => unreachable!(),
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

    #[test]
    fn insert() {
        let mut tree = Tree::Empty;

        tree = tree.insert("A".to_string());
        if let NonEmpty(ref node) = tree {
            assert_eq!(node.color, Black);
        }

        tree = tree.insert("E".to_string());
        assert_eq!(tree.right_is_red(), true);

        //     A
        //      \              E
        //      (E)     ->    / \
        //        \          A   S
        //        (S)
        tree = tree.insert("S".to_string());
        if let NonEmpty(node) = &tree {
            assert_eq!(node.element, "E".to_string());
            assert_eq!(node.color, Black);
            if let (NonEmpty(lnode), NonEmpty(rnode)) = (&node.left, &node.right) {
                assert_eq!(lnode.element, "A".to_string());
                assert_eq!(lnode.color, Black);
                assert_eq!(rnode.element, "S".to_string());
                assert_eq!(rnode.color, Black);
            } else {
                panic!("Left and right nodes should not be Empty");
            }
        } else {
            panic!("Node can't be Empty");
        }
    }
}
