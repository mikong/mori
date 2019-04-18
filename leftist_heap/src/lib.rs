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

impl<T> Heap<T> {
    fn rank(&self) -> usize {
        match self {
            Heap::Empty => 0,
            Heap::NonEmpty(ref node) => node.rank,
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Heap::Empty => true,
            _ => false,
        }
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
