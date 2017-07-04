#[cfg(test)]

use merkletree::MerkleTree;
use treeelement::TreeElement;
use utils::{Hashable, Hasher};

#[test]
fn produce_tree_from_one_elem() {
    let data = vec![format!("one")];
    let test_hash = Hasher::hash_leaf_data(data[0].get_bytes());

    let tree = MerkleTree::from_vector(data).unwrap();

    assert_eq!(tree.count(), 1);
    assert!(!tree.is_empty());
    assert_eq!(tree.height(), 0);
    assert_eq!(tree.root_hash(), test_hash.as_slice());
}

#[test]
fn produce_tree_from_two_elems() {
    let data = vec!["one".to_string(), "two".to_string()];

    // construct the expected hash value
    let hash_left = Hasher::hash_leaf_data(data[0].get_bytes());
    let hash_right = Hasher::hash_leaf_data(data[1].get_bytes());
    let test_hash = Hasher::hash_node_data(hash_left.as_slice(), hash_right.as_slice());

    let tree = MerkleTree::from_vector(data).unwrap();

    assert_eq!(tree.count(), 2);
    assert!(!tree.is_empty());
    assert_eq!(tree.height(), 1);
    assert_eq!(tree.root_hash(), test_hash.as_slice());
}

#[test]
fn handle_empty_array() {
    let data: Vec<String> = Vec::new();

    let tree = MerkleTree::from_vector(data);

    assert!(tree.is_none());
}

#[test]
fn check_leaf_hash() {
    let data = format!("test string");
    let hash = Hasher::hash_leaf_data(data.get_bytes()); 

    let leaf = TreeElement::new_leaf(data);

    assert_eq!(leaf.hash(), hash.as_slice());
}

#[test]
fn check_nodes_hash() {
    let d1 = format!("first leaf");
    let d2 = format!("second leaf");
    let h1 = Hasher::hash_leaf_data(d1.get_bytes()); 
    let h2 = Hasher::hash_leaf_data(d2.get_bytes());
    let h12 = Hasher::hash_node_data(h1.as_slice(), h2.as_slice());

    let node = TreeElement::new_node(TreeElement::new_leaf(d1), TreeElement::new_leaf(d2));

    assert_eq!(node.hash(), h12.as_slice());
}