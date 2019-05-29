use merkle_tree::MerkleTree;
use sha2::{Sha256, Digest};

#[test]
fn it_validates() {
    let data = ["A", "BC", "DEF"];
    let hbc = Sha256::digest(b"BC");

    // Calls all public API functions of MerkleTree
    let tree = MerkleTree::build(&data);
    let proof = tree.get_proof(1);
    let root = tree.root_hash();
    assert!(MerkleTree::validate(hbc, proof, root));
}
