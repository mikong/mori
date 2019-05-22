use std::mem;

use sha2::{Sha256, Digest};
use sha2::digest::generic_array::GenericArray;
use sha2::digest::generic_array::typenum::U32;
use sha2::digest::generic_array::sequence::Concat;

#[derive(Debug)]
pub enum MerkleTree {
    Empty,
    NonEmpty(Box<Node>),
}

#[derive(Debug)]
pub struct Node {
    element: GenericArray<u8, U32>,
    left: MerkleTree,
    right: MerkleTree,
}

impl MerkleTree {
    pub fn build<T: AsRef<[u8]>>(data: &Vec<T>) -> MerkleTree {
        let mut leaf_nodes = data.iter().map(|val| {
            let hash = Sha256::digest(val.as_ref());

            MerkleTree::NonEmpty(Box::new(Node {
                element: hash,
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

            let value = match (&left, &right) {
                (MerkleTree::NonEmpty(l), MerkleTree::NonEmpty(r)) => {
                    l.element.concat(r.element)
                },
                (_, _) => unreachable!(),
            };
            let hash = Sha256::digest(&value);

            let tree = MerkleTree::NonEmpty(Box::new(Node {
                element: hash,
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
    use super::MerkleTree::*;

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
        if let NonEmpty(node) = tree {
            assert_eq!(node.element, GenericArray::clone_from_slice(&root_hash));
            if let (NonEmpty(lnode), NonEmpty(rnode)) = (&node.left, &node.right) {
                assert_eq!(lnode.element, GenericArray::clone_from_slice(&habcd));
                assert_eq!(rnode.element, GenericArray::clone_from_slice(&he));
            }
        } else {
            panic!("Tree can't be empty");
        }
    }
}
