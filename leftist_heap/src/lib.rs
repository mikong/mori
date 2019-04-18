#[derive(Debug)]
enum Heap<T> {
    Empty,
    NonEmpty(Box<Node<T>>),
}

#[derive(Debug)]
struct Node<T> {
    rank: usize,
    element: T,
    left: Heap<T>,
    right: Heap<T>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::Heap::*;

    #[test]
    fn it_works() {
        let _heap = NonEmpty(Box::new(Node {
            rank: 0,
            element: 5,
            left: Empty,
            right: Empty,
        }));
    }
}
