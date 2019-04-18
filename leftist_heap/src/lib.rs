#[derive(Debug)]
pub enum Heap<T> {
    Empty,
    NonEmpty(Box<Node<T>>),
}

#[derive(Debug)]
pub struct Node<T> {
    rank: usize,
    element: T,
    left: Heap<T>,
    right: Heap<T>,
}

impl<T: Ord> Heap<T> {
    fn rank(&self) -> usize {
        match self {
            Heap::Empty => 0,
            Heap::NonEmpty(ref node) => node.rank,
        }
    }

    fn make(element: T, left: Heap<T>, right: Heap<T>) -> Heap<T> {
        if left.rank() >= right.rank() {
            Heap::NonEmpty(Box::new(Node {
                rank: right.rank() + 1,
                element,
                left,
                right,
            }))
        } else {
            Heap::NonEmpty(Box::new(Node {
                rank: left.rank() + 1,
                element,
                left: right,
                right: left,                
            }))
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Heap::Empty => true,
            _ => false,
        }
    }

    fn merge(a: Heap<T>, b: Heap<T>) -> Heap<T> {
        match (a, b) {
            (h, Heap::Empty) => h,
            (Heap::Empty, h) => h,
            (Heap::NonEmpty(h1), Heap::NonEmpty(h2)) => {
                if h1.element <= h2.element {
                    Heap::make(
                        h1.element,
                        h1.left,
                        Heap::merge(h1.right, Heap::NonEmpty(h2))
                    )
                } else {
                    Heap::make(
                        h2.element,
                        h2.left,
                        Heap::merge(Heap::NonEmpty(h1), h2.right)
                    )
                }
            },
        }
    }

    pub fn insert(self, x: T) -> Heap<T> {
        let new_node = Box::new(Node {
            rank: 1,
            element: x,
            left: Heap::Empty,
            right: Heap::Empty,
        });
        Heap::merge(Heap::NonEmpty(new_node), self)
    }

    pub fn find_min(&self) -> Option<&T> {
        match self {
            Heap::NonEmpty(ref node) => Some(&node.element),
            Heap::Empty => None,
        }
    }

    pub fn delete_min(self) -> Heap<T> {
        match self {
            Heap::NonEmpty(node) => {
                Heap::merge(node.left, node.right)
            },
            Heap::Empty => Heap::Empty,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::Heap::*;

    #[test]
    fn it_works() {
        let mut heap = Empty;
        heap = heap.insert(5);
        assert_eq!(heap.rank(), 1);
        assert_eq!(heap.is_empty(), false);
        assert_eq!(heap.find_min(), Some(&5));

        heap = heap.insert(10);
        assert_eq!(heap.find_min(), Some(&5));
        assert_eq!(heap.rank(), 1);
        if let NonEmpty(ref node) = heap {
            assert_eq!(node.left.rank(), 1);
            if let NonEmpty(ref left_node) = node.left {
                assert_eq!(left_node.element, 10);
            }
        } else {
            panic!("Node can't be Empty");
        }

        heap = heap.insert(15);
        assert_eq!(heap.rank(), 2);
        heap = heap.insert(1);
        assert_eq!(heap.rank(), 1);
        assert_eq!(heap.find_min(), Some(&1));

        heap = heap.delete_min();
        assert_eq!(heap.find_min(), Some(&5));
        heap = heap.delete_min();
        assert_eq!(heap.find_min(), Some(&10));
    }

    #[test]
    fn emptiness() {
        let heap: Heap<u32> = Empty;
        assert_eq!(heap.is_empty(), true);
    }

    #[test]
    fn children_swap() {
        let mut heap = Empty;

        //    5
        //   /
        // 10
        heap = heap.insert(5).insert(10);
        if let NonEmpty(ref node) = heap {
            assert_eq!(node.left.find_min(), Some(&10));
            assert_eq!(node.right.find_min(), None);
        };

        //       5
        //      / \
        //    15   10
        //   /  \
        //  20  25
        heap = heap.insert(15).insert(20).insert(25);
        if let NonEmpty(ref node) = heap {
            assert_eq!(node.left.find_min(), Some(&15));
            assert_eq!(node.right.find_min(), Some(&10));
        }
    }
}
