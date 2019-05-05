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

impl<T> Heap<T> {
    pub fn find_min(&self) -> Option<&T> {
        match self {
            Heap::NonEmpty(node) => Some(&node.element),
            Heap::Empty => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::Heap::*;

    #[test]
    fn it_works() {
        let _heap = NonEmpty(Box::new(Node {
            element: 5,
            list: vec![],
        }));
    }

    #[test]
    fn find_min() {
        unimplemented!();
    }
}
