use crypto_hash::{digest, Algorithm};

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
        digest(Algorithm::SHA256, data)
    }

    /// Produces hash of Hashable using SHA256
    pub fn hash_leaf<T: Hashable>(data: &T) -> Vec<u8> {
        digest(Algorithm::SHA256, data.get_bytes())
    }

    /// Produces hash of two nodes using SHA256
    pub fn hash_node_data(left: &[u8], right: &[u8]) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::with_capacity(left.len() + right.len());
        buffer.extend(left);
        buffer.extend(right);

        digest(Algorithm::SHA256, buffer.as_ref())
    }
}
