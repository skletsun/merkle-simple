use crypto_hash::{digest, Algorithm};

// value to seed leaf's hash
static LEAF: &'static [u8] = b"LEAF";

// value to seed node's hash
static NODE: &'static [u8] = b"NODE";

/// This trait should be implemented by data structures in order to add them to tree
pub trait Hashable {
    /// Provides the representation of a data structure as an array of bytes
    fn get_bytes(&self) -> &[u8];
}

/// Implementation of Hashable for String
impl Hashable for String {
    fn get_bytes(&self) -> &[u8] {
        &self.as_bytes()
    }
}
/// Helper structure with single purpose to serve hashing of data.
/// The SHA256 algorithm has been chosen for the sake of simplisity.
#[derive(Debug, Clone, Copy)]
pub struct Hasher;

impl Hasher {

    /// Produces hash of data using SHA256
    pub fn hash_leaf_data(data: &[u8]) -> Vec<u8> {
        Hasher::gen_leaf_hash(data)
    }

    /// Produces hash of Hashable using SHA256
    pub fn hash_leaf<T: Hashable>(data: &T) -> Vec<u8> {
        Hasher::gen_leaf_hash(data.get_bytes())
    }

    // Helper method that actually produces hash for leaf
    fn gen_leaf_hash(data: &[u8]) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::with_capacity(LEAF.len() + data.len());
        buffer.extend(LEAF);
        buffer.extend(data);

        digest(Algorithm::SHA256, buffer.as_ref())
    }

    /// Produces hash of two nodes using SHA256
    pub fn hash_node_data(left: &[u8], right: &[u8]) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::with_capacity(NODE.len() + left.len() + right.len());
        buffer.extend(NODE);
        buffer.extend(left);
        buffer.extend(right);

        digest(Algorithm::SHA256, buffer.as_ref())
    }
}

#[test]
fn leaf_hash_shouldnt_match_node() {
    // prepare data
    let test_hash_source_node = b"This is a test value for hashing";

    // we just concatenate source data of nodes in order to get leaf hash source
    let mut buffer = Vec::with_capacity(test_hash_source_node.len() * 2);
    buffer.extend(test_hash_source_node);
    buffer.extend(test_hash_source_node);

    let test_hash_source_leaf = buffer.as_ref();

    // produce leaf and node from same data
    let leaf_hash = Hasher::hash_leaf_data(test_hash_source_leaf);
    let node_hash = Hasher::hash_node_data(test_hash_source_node, test_hash_source_node);

    assert!(leaf_hash != node_hash);
}
