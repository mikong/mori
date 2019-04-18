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
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::Heap::*;

    #[test]
    fn it_works() {
        let heap = NonEmpty(Box::new(Node {
            rank: 0,
            element: 5,
            left: Empty,
            right: Empty,
        }));

        assert_eq!(heap.rank(), 0);
        assert_eq!(heap.is_empty(), false);
    }

    #[test]
    fn emptiness() {
        let heap: Heap<u32> = Empty;
        assert_eq!(heap.is_empty(), true);
    }
}
