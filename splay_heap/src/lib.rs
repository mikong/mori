#[derive(Debug)]
pub enum Heap<T> {
    Empty,
    NonEmpty(Box<Node<T>>),
}

#[derive(Debug)]
pub struct Node<T> {
    element: T,
    left: Heap<T>,
    right: Heap<T>,
}

impl<T: Ord> Heap<T> {
    pub fn new(element: T, left: Heap<T>, right: Heap<T>) -> Heap<T> {
        Heap::NonEmpty(Box::new(Node {
            element,
            left,
            right,
        }))
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Heap::Empty => true,
            _ => false,
        }
    }

    pub fn find_min(&self) -> Option<&T> {
        match self {
            Heap::NonEmpty(ref node) => {
                if let Heap::Empty = node.left {
                    return Some(&node.element);
                }
                node.left.find_min()                
            },
            Heap::Empty => None,
        }
    }

    pub fn delete_min(self) -> Heap<T> {
        match self {
            Heap::Empty => Heap::Empty,
            Heap::NonEmpty(node) => {
                match node.left {
                    Heap::Empty => node.right,
                    Heap::NonEmpty(lnode) => {
                        let h = Heap::new(node.element, lnode.right, node.right);
                        if lnode.left.is_empty() {
                            h
                        } else {
                            Heap::new(lnode.element, lnode.left.delete_min(), h)
                        }
                    },
                }
            },
        }
    }

    pub fn insert(self, element: T) -> Heap<T> {
        let (smaller, bigger) = self.partition(&element);
        Heap::new(element, smaller, bigger)
    }

    fn partition(self, pivot: &T) -> (Heap<T>, Heap<T>) {
        match self {
            Heap::NonEmpty(node) => {
                if node.element <= *pivot {
                    match node.right {
                        Heap::NonEmpty(rnode) => {
                            if rnode.element <= *pivot {
                                let (small, big) = rnode.right.partition(pivot);
                                let ss = Heap::new(node.element, node.left, rnode.left);
                                let s = Heap::new(rnode.element, ss, small);
                                (s, big)
                            } else {
                                let (small, big) = rnode.left.partition(pivot);
                                let s = Heap::new(node.element, node.left, small);
                                let b = Heap::new(rnode.element, big, rnode.right);
                                (s, b)
                            }
                        },
                        Heap::Empty => (Heap::NonEmpty(node), Heap::Empty),
                    }
                } else {
                    match node.left {
                        Heap::NonEmpty(lnode) => {
                            if lnode.element <= *pivot {
                                let (small, big) = lnode.right.partition(pivot);
                                let s = Heap::new(lnode.element, lnode.left, small);
                                let b = Heap::new(node.element, big, node.right);
                                (s, b)
                            } else {
                                let (small, big) = lnode.left.partition(pivot);
                                let bb = Heap::new(node.element, lnode.right, node.right);
                                let b = Heap::new(lnode.element, big, bb);
                                (small, b)
                            }
                        },
                        Heap::Empty => (Heap::Empty, Heap::NonEmpty(node)),
                    }
                }
            },
            Heap::Empty => (Heap::Empty, Heap::Empty),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::Heap::*;

    #[test]
    fn it_works() {
        let _heap = Heap::new(5, Empty, Empty);
    }

    #[test]
    fn is_empty() {
        let mut heap = Empty;
        assert_eq!(heap.is_empty(), true);

        heap = heap.insert(10);
        assert_eq!(heap.is_empty(), false);
    }

    #[test]
    fn find_min() {
        let mut heap = Empty;
        assert_eq!(heap.find_min(), None);

        heap = heap.insert(20).insert(30).insert(25).insert(10).insert(15);
        assert_eq!(heap.find_min(), Some(&10));
    }

    #[test]
    fn delete_min() {
        let mut heap: Heap<u32> = Empty;
        heap = heap.delete_min();
        assert!(heap.is_empty());

        //   10
        //     \
        //     15
        heap = heap.insert(15).insert(10);
        assert_eq!(heap.find_min(), Some(&10));
        heap = heap.delete_min();
        assert_eq!(heap.find_min(), Some(&15));
    }

    #[test]
    fn insert() {
        let mut heap = Empty;

        //      30
        //     /
        //    20
        //   /
        //  10
        heap = heap.insert(10).insert(20).insert(30);
        if let NonEmpty(ref node) = heap {
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
        heap = heap.insert(25);
        if let NonEmpty(ref node) = heap {
            assert_eq!(node.element, 25);
            if let (NonEmpty(lnode), NonEmpty(rnode)) = (&node.left, &node.right) {
                assert_eq!(lnode.element, 20);
                assert_eq!(rnode.element, 30);
            }
        }

        //      15
        //     /  \
        //    10  20
        //          \
        //          25
        //            \
        //            30
        heap = heap.insert(15);
        if let NonEmpty(ref node) = heap {
            assert_eq!(node.element, 15);
            if let (NonEmpty(lnode), NonEmpty(rnode)) = (&node.left, &node.right) {
                assert_eq!(lnode.element, 10);
                assert_eq!(rnode.element, 20);
            }
        }

        //       27
        //      /  \
        //     20   30
        //    /  \
        //  15   25
        //  /
        // 10
        heap = heap.insert(27);
        if let NonEmpty(ref node) = heap {
            assert_eq!(node.element, 27);
            if let (NonEmpty(lnode), NonEmpty(rnode)) = (&node.left, &node.right) {
                assert_eq!(lnode.element, 20);
                assert_eq!(rnode.element, 30);
                if let (NonEmpty(ll), NonEmpty(lr)) = (&lnode.left, &lnode.right) {
                    assert_eq!(ll.element, 15);
                    assert_eq!(lr.element, 25);
                }
            }
        }
    }
}
