extern crate merkle_simple;

use merkle_simple::{MerkleTree, Hashable, Hasher};

#[derive(Debug, Clone)]
struct Transaction {
    data: Vec<u8>
}

impl Transaction {

    fn new(data: &[u8]) -> Self {
        Transaction {
            data: data.to_vec()
        }
    }
}

impl Hashable for Transaction {
    fn get_bytes(&self) -> &[u8] {
        &self.data
    }
}

#[test]
fn build_tree_for_custom_struct() {
    let t1 = Transaction::new(b"transaction #1");
    let t2 = Transaction::new(b"transaction #2");
    let t3 = Transaction::new(b"transaction #3");
    let t4 = Transaction::new(b"transaction #4");

    let tree_wrapper = MerkleTree::from_vector(vec![t1.clone(), t2.clone(), t3.clone(), t4.clone()]);

    assert!(tree_wrapper.is_some());

    let tree = tree_wrapper.unwrap();
    assert_eq!(tree.count(), 4);
    assert!(!tree.is_empty());
    assert_eq!(tree.height(), 2);

    // verify hashes
    let h1 = Hasher::hash_leaf_data(t1.get_bytes());
    let h2 = Hasher::hash_leaf_data(t2.get_bytes());
    let h3 = Hasher::hash_leaf_data(t3.get_bytes());
    let h4 = Hasher::hash_leaf_data(t4.get_bytes());

    let h12 = Hasher::hash_node_data(&h1, &h2);
    let h34 = Hasher::hash_node_data(&h3, &h4);

    let root_hash = Hasher::hash_node_data(&h12, &h34);
    assert_eq!(tree.root_hash(), root_hash.as_slice());
}