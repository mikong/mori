use std::mem;

#[derive(Debug)]
pub enum MerkleTree {
    Empty,
    NonEmpty(Box<Node>),
}

#[derive(Debug)]
pub struct Node {
    element: Vec<u8>,
    left: MerkleTree,
    right: MerkleTree,
}

impl MerkleTree {
    pub fn build<T>(data: &Vec<T>) -> MerkleTree {
        let mut leaf_nodes = data.iter().map(|_val| {
            MerkleTree::NonEmpty(Box::new(Node {
                element: vec![],
                left: MerkleTree::Empty,
                right: MerkleTree::Empty,
            }))
        }).collect();

        MerkleTree::build_tree(&mut leaf_nodes)
    }

    fn build_tree(nodes: &mut Vec<MerkleTree>) -> MerkleTree {
        let mut new_nodes = vec![];

        for pair in nodes.chunks_exact_mut(2) {
            let mut left = MerkleTree::Empty;
            let mut right = MerkleTree::Empty;
            mem::swap(&mut left, &mut pair[0]);
            mem::swap(&mut right, &mut pair[1]);

            let tree = MerkleTree::NonEmpty(Box::new(Node {
                element: vec![],
                left: left,
                right: right,
            }));

            new_nodes.push(tree);
        }

        if nodes.len() % 2 == 1 {
            new_nodes.push(nodes.pop().unwrap());
        }

        if new_nodes.len() == 1 {
            return new_nodes.pop().unwrap();
        }

        MerkleTree::build_tree(&mut new_nodes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let data = vec![1, 2, 3, 4, 5];
        let _tree = MerkleTree::build(&data);
    }
}
