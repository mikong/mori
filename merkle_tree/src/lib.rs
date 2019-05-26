use std::mem;

use sha2::{Sha256, Digest};
use sha2::digest::generic_array::GenericArray;
use sha2::digest::generic_array::typenum::U32;
use sha2::digest::generic_array::sequence::Concat;

#[derive(Debug, PartialEq)]
pub enum Position {
    Left,
    Right,
}

#[derive(Debug)]
pub enum MerkleTree {
    Empty,
    NonEmpty(Box<Node>),
}

#[derive(Debug)]
pub struct Node {
    element: GenericArray<u8, U32>,
    leaf_count: usize,
    left: MerkleTree,
    right: MerkleTree,
}

impl MerkleTree {
    fn new(
        element: GenericArray<u8, U32>,
        leaf_count: usize,
        left: MerkleTree,
        right: MerkleTree
    ) -> MerkleTree {
        MerkleTree::NonEmpty(Box::new(Node {
            element,
            leaf_count,
            left,
            right,
        }))
    }

    /// Creates a `MerkleTree` from a slice of `data`.
    ///
    /// # Panics
    ///
    /// Panics if `data.len()` is 0.
    pub fn build<T: AsRef<[u8]>>(data: &[T]) -> MerkleTree {
        if data.len() == 0 {
            panic!("Merkle tree can't be empty: the len is 0");
        }

        let mut leaf_nodes = data.iter().map(|val| {
            let hash = Sha256::digest(val.as_ref());

            MerkleTree::new(hash, 1, MerkleTree::Empty, MerkleTree::Empty)
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

            let hash = MerkleTree::concat_and_hash(&left, &right);
            let leaf_count = left.leaf_count() + right.leaf_count();
            let tree = MerkleTree::new(hash, leaf_count, left, right);

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

    fn concat_and_hash(left: &MerkleTree, right: &MerkleTree) -> GenericArray<u8, U32> {
        let value = match (&left, &right) {
            (MerkleTree::NonEmpty(l), MerkleTree::NonEmpty(r)) => {
                l.element.concat(r.element)
            },
            (_, _) => unreachable!(),
        };

        Sha256::digest(&value)
    }

    fn leaf_count(&self) -> usize {
        match self {
            MerkleTree::NonEmpty(n) => n.leaf_count,
            MerkleTree::Empty => 0,
        }
    }

    /// Returns the hashes needed to verify the data at `index`.
    ///
    /// # Panics
    ///
    /// Panics if `index` is out of bounds.
    pub fn get_proof(&self, index: usize) -> Vec<(Position, GenericArray<u8, U32>)> {
        if index >= self.leaf_count() {
            panic!(
                "index out of bounds: the len is {} but the index is {}",
                self.leaf_count(),
                index
            );
        }

        let mut stack = Vec::new();
        let mut current = self;
        let mut base = 0;

        use MerkleTree::NonEmpty;
        while current.leaf_count() > 1 {
            if let NonEmpty(node) = current {
                if let (NonEmpty(l), NonEmpty(r)) = (&node.left, &node.right) {
                    if index < l.leaf_count + base {
                        stack.push((Position::Right, r.element));
                        current = &node.left;
                    } else {
                        base += l.leaf_count;
                        stack.push((Position::Left, l.element));
                        current = &node.right;
                    }
                }
            }
        }

        stack.reverse();
        stack
    }

    /// Returns the root hash of the tree.
    ///
    /// # Panics
    ///
    /// Panics if `MerkleTree` is `Empty`.
    pub fn root_hash(&self) -> GenericArray<u8, U32> {
        match self {
            MerkleTree::NonEmpty(node) => node.element,
            MerkleTree::Empty => panic!("Merkle tree can't be empty"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::MerkleTree::*;

    #[test]
    #[should_panic]
    fn zero_element() {
        let data: [String; 0] = [];
        MerkleTree::build(&data);
    }

    #[test]
    fn small_trees() {
        let ha = Sha256::digest(b"A");
        let hb = Sha256::digest(b"B");
        let hc = Sha256::digest(b"C");
        let hab = Sha256::digest(&ha.concat(hb));
        let habc = Sha256::digest(&hab.concat(hc));

        // smallest: one-element tree
        let data = ["A"];
        let tree = MerkleTree::build(&data);
        assert_eq!(tree.root_hash(), GenericArray::clone_from_slice(&ha));

        // smallest with children: two-element tree
        let data = ["A", "B"];
        let tree = MerkleTree::build(&data);
        assert_eq!(tree.root_hash(), GenericArray::clone_from_slice(&hab));

        // smallest unbalanced: three-element tree
        let data = ["A", "B", "C"];
        let tree = MerkleTree::build(&data);
        assert_eq!(tree.root_hash(), GenericArray::clone_from_slice(&habc));
    }

    #[test]
    fn it_works() {
        let data = vec!["A", "B", "C", "D", "E"];
        let tree = MerkleTree::build(&data);

        let ha = Sha256::digest(b"A");
        let hb = Sha256::digest(b"B");
        let hc = Sha256::digest(b"C");
        let hd = Sha256::digest(b"D");
        let he = Sha256::digest(b"E");

        let hab = Sha256::digest(&ha.concat(hb));
        let hcd = Sha256::digest(&hc.concat(hd));
        let habcd = Sha256::digest(&hab.concat(hcd));

        let root_hash = Sha256::digest(&habcd.concat(he));

        //                                  root_hash = H(habcd + he)
        //                                   /                    \
        //                    habcd = H(hab + hcd)               he = H(E)
        //                     /                \
        //       hab = H(ha + hb)            hcd = H(hc + hd)
        //       /             \             /             \
        //   ha = H("A")    hb = H("B")  hc = H("C")    hd = H("D")
        assert_eq!(tree.root_hash(), GenericArray::clone_from_slice(&root_hash));
        if let NonEmpty(node) = tree {
            if let (NonEmpty(lnode), NonEmpty(rnode)) = (&node.left, &node.right) {
                assert_eq!(lnode.element, GenericArray::clone_from_slice(&habcd));
                assert_eq!(rnode.element, GenericArray::clone_from_slice(&he));
            }
        } else {
            panic!("Tree can't be empty");
        }
    }

    #[test]
    fn leaf_count() {
        let tree = MerkleTree::Empty;
        assert_eq!(tree.leaf_count(), 0);

        let data = ["A"];
        let tree = MerkleTree::build(&data);
        assert_eq!(tree.leaf_count(), 1);

        let data = ["A", "B", "C", "D", "E"];
        let tree = MerkleTree::build(&data);
        assert_eq!(tree.leaf_count(), 5);
    }

    #[test]
    fn get_proof() {
        let data = ["A", "B", "C", "D", "E"];
        let tree = MerkleTree::build(&data);
        let proof = tree.get_proof(2);

        let ha = Sha256::digest(b"A");
        let hb = Sha256::digest(b"B");
        let hd = Sha256::digest(b"D");
        let he = Sha256::digest(b"E");
        let hab = Sha256::digest(&ha.concat(hb));

        //                root
        //              //    \
        //          habcd      he
        //         /     \\
        //      hab       hcd
        //     /   \     //  \
        //   ha     hb  hc    hd
        //
        // The path to index 2 or "C" is marked with // or \\. To
        // verify, we need the hashes from the siblings of the nodes
        // in the path: hd, hab, and he.
        let mut proof_iter = proof.iter();

        let (pos, hash) = proof_iter.next().unwrap();
        assert_eq!(pos, &Position::Right);
        assert_eq!(*hash, GenericArray::clone_from_slice(&hd));

        let (pos, hash) = proof_iter.next().unwrap();
        assert_eq!(pos, &Position::Left);
        assert_eq!(*hash, GenericArray::clone_from_slice(&hab));

        let (pos, hash) = proof_iter.next().unwrap();
        assert_eq!(pos, &Position::Right);
        assert_eq!(*hash, GenericArray::clone_from_slice(&he));
    }
}
