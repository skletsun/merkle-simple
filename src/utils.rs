use crypto_hash::{digest, Algorithm};

pub trait Hashable {
    fn get_bytes(&self) -> &[u8];
}

impl Hashable for String {
     fn get_bytes(&self) -> &[u8] {
         &self.as_bytes()
     }
}

#[derive(Debug)]
pub struct Utils;

impl Utils {
    pub fn hash_leaf_data(data: &[u8]) -> Vec<u8> {
        digest(Algorithm::SHA256, data)
    }

    pub fn hash_node_data(left: &[u8], right: &[u8]) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::with_capacity(left.len() + right.len());
        buffer.extend(left);
        buffer.extend(right);

        digest(Algorithm::SHA256, buffer.as_ref())
    }
}