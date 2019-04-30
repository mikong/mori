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

impl<T: Ord> Tree<T> {
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

    fn partition(self, pivot: &T) -> (Tree<T>, Tree<T>) {
        match self {
            Tree::NonEmpty(node) => {
                if node.element <= *pivot {
                    match node.right {
                        Tree::NonEmpty(rnode) => {
                            if rnode.element <= *pivot {
                                let (small, big) = rnode.right.partition(pivot);
                                let ss = Tree::new(node.element, node.left, rnode.left);
                                let s = Tree::new(rnode.element, ss, small);
                                (s, big)
                            } else {
                                let (small, big) = rnode.left.partition(pivot);
                                let s = Tree::new(node.element, node.left, small);
                                let b = Tree::new(rnode.element, big, rnode.right);
                                (s, b)
                            }
                        },
                        Tree::Empty => (Tree::NonEmpty(node), Tree::Empty),
                    }
                } else {
                    match node.left {
                        Tree::NonEmpty(lnode) => {
                            if lnode.element <= *pivot {
                                let (small, big) = lnode.right.partition(pivot);
                                let s = Tree::new(lnode.element, lnode.left, small);
                                let b = Tree::new(node.element, big, node.right);
                                (s, b)
                            } else {
                                let (small, big) = lnode.left.partition(pivot);
                                let bb = Tree::new(node.element, lnode.right, node.right);
                                let b = Tree::new(lnode.element, big, bb);
                                (small, b)
                            }
                        },
                        Tree::Empty => (Tree::Empty, Tree::NonEmpty(node)),
                    }
                }
            },
            Tree::Empty => (Tree::Empty, Tree::Empty),
        }
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

    #[test]
    fn insert() {
        let mut tree = Empty;

        //      30
        //     /
        //    20
        //   /
        //  10
        tree = tree.insert(10).insert(20).insert(30);
        if let NonEmpty(ref node) = tree {
            assert_eq!(node.element, 30);
            if let NonEmpty(lnode) = &node.left {
                assert_eq!(lnode.element, 20);
            }
        }

        //      25
        //     /  \
        //    20  30
        //   /
        //  10
        tree = tree.insert(25);
        if let NonEmpty(ref node) = tree {
            assert_eq!(node.element, 25);
            if let (NonEmpty(lnode), NonEmpty(rnode)) = (&node.left, &node.right) {
                assert_eq!(lnode.element, 20);
                assert_eq!(rnode.element, 30);
            }
        }
    }
}
