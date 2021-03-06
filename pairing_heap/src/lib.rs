#[derive(Debug)]
pub enum Heap<T> {
    Empty,
    NonEmpty(Box<Node<T>>),
}

#[derive(Debug)]
pub struct Node<T> {
    element: T,
    list: Vec<Heap<T>>,
}

impl<T: Ord> Heap<T> {
    pub fn new(element: T, list: Vec<Heap<T>>) -> Heap<T> {
        Heap::NonEmpty(Box::new(Node {
            element,
            list,
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
            Heap::NonEmpty(node) => Some(&node.element),
            Heap::Empty => None,
        }
    }

    pub fn merge(a: Heap<T>, b: Heap<T>) -> Heap<T> {
        match (a, b) {
            (h, Heap::Empty) => h,
            (Heap::Empty, h) => h,
            (Heap::NonEmpty(mut h1), Heap::NonEmpty(mut h2)) => {
                // The tree with the larger root should become
                // the leftmost child of the tree with the smaller
                // root. As an optimization, we can treat the list
                // as if it's in reverse order so we only need to
                // push to the end of the Vec.
                if h1.element <= h2.element {
                    h1.list.push(Heap::NonEmpty(h2));
                    Heap::NonEmpty(h1)
                } else {
                    h2.list.push(Heap::NonEmpty(h1));
                    Heap::NonEmpty(h2)
                }
            },
        }
    }

    pub fn insert(self, x: T) -> Heap<T> {
        let h = Heap::new(x, vec![]);
        Heap::merge(h, self)
    }

    fn merge_pairs(mut list: Vec<Heap<T>>) -> Heap<T> {
        if list.is_empty() {
            Heap::Empty
        } else if list.len() == 1 {
            list.pop().unwrap()
        } else {
            let h1 = list.pop().unwrap();
            let h2 = list.pop().unwrap();
            Heap::merge(Heap::merge(h1, h2), Heap::merge_pairs(list))
        }
    }

    pub fn delete_min(self) -> Heap<T> {
        match self {
            Heap::Empty => Heap::Empty,
            Heap::NonEmpty(node) => {
                Heap::merge_pairs(node.list)
            },
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
        assert_eq!(heap.find_min(), Some(&5));
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
        let mut heap = Empty;
        heap = heap.delete_min();
        assert_eq!(heap.is_empty(), true);

        heap = heap.insert(20).insert(10).insert(15);
        assert_eq!(heap.find_min(), Some(&10));
        heap = heap.delete_min();
        assert_eq!(heap.find_min(), Some(&15));
    }

    #[test]
    fn merge() {
        let mut heap = Heap::new(5, vec![]);

        heap = Heap::merge(heap, Empty);
        if let Heap::NonEmpty(ref node) = heap {
            assert_eq!(node.element, 5);
        } else {
            panic!("Heap can't be Empty");
        }

        heap = Heap::merge(Empty, heap);
        if let Heap::NonEmpty(ref node) = heap {
            assert_eq!(node.element, 5);
        } else {
            panic!("Heap can't be Empty");
        }

        let h2 = Heap::new(10, vec![]);
        heap = Heap::merge(heap, h2);
        if let Heap::NonEmpty(ref node) = heap {
            assert_eq!(node.element, 5);
            if let Heap::NonEmpty(ref child) = node.list.first().unwrap() {
                assert_eq!(child.element, 10);
            } else {
                panic!("List element can't be Empty");
            }
        } else {
            panic!("Heap can't be Empty");
        }
    }
}
