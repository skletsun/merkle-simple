#[cfg(test)]

use merkletree::MerkleTree;
use utils::Utils;
use utils::Hashable;

#[test]
fn produce_tree_from_one_elem() {
    let data = vec![format!("one")];
    let test_hash = Utils::hash_leaf_data(data[0].get_bytes());

    let tree = MerkleTree::from_vector(data).unwrap();

    assert_eq!(tree.count(), 1);
    assert!(!tree.is_empty());
    assert_eq!(tree.height(), 0);
    assert_eq!(tree.root_hash(), test_hash.as_slice());
}

#[test]
fn produce_tree_from_two_elems() {
    let data = vec!["one".to_string(), "two".to_string()];
    let test_hash = Utils::hash_node_data(data[0].get_bytes(), data[1].get_bytes());

    let tree = MerkleTree::from_vector(data).unwrap();

    assert_eq!(tree.count(), 2);
    assert!(!tree.is_empty());
    assert_eq!(tree.height(), 1);
    // assert_eq!(tree.root_hash(), test_hash.as_slice());
}
