extern crate merkle_simple;

#[allow(unused_imports)]
use merkle_simple::{MerkleTree, Hashable};

#[test]
fn test_inclusion_success() {
    let data = vec![format!("one"), format!("two"), format!("three"), format!("four")];
    let tree = MerkleTree::from_vector(data.clone()).unwrap();

    for data_to_check in data {
        let proof = tree.get_proof(data_to_check);
        assert!(proof.is_some());

        let result = proof.unwrap().validate(tree.root_hash());
        assert!(result);
    }
}

#[test]
fn test_inclusion_fail() {
    let data = vec![format!("one"), format!("two"), format!("three"), format!("four")];
    let tree = MerkleTree::from_vector(data).unwrap();

    let data_to_check = format!("five");

    let proof = tree.get_proof(data_to_check);
    assert!(proof.is_none());
}