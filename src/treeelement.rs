use utils::{Hashable, Utils};
// use crypto_hash::{digest, Algorithm};

#[derive(Debug)]
pub enum TreeElement <T> where T: Hashable {
    Leaf {
        value: T,
        hash: Vec<u8>
    },
    Node {
        left: Box<TreeElement<T>>,
        right: Box<TreeElement<T>>,
        hash: Vec<u8>
    }
}

impl <T> TreeElement <T> where T: Hashable {

    pub fn hash(&self) -> &[u8] {
        match *self {
            TreeElement::Leaf { ref hash, .. } => hash,
            TreeElement::Node { ref hash, .. } => hash
        }
    }

    pub fn new_leaf(value: T) -> TreeElement <T> {
        TreeElement::Leaf {
//            hash: digest(Algorithm::SHA256, value.get_bytes()),
            hash: Utils::hash_leaf_data(value.get_bytes()),
            value: value
        }
    }

    pub fn new_node(left: TreeElement<T>, right: TreeElement<T>) -> TreeElement <T> {
        TreeElement::Node {
            // hash: digest(Algorithm::SHA256, &buffer),
            hash: Utils::hash_node_data(left.hash(), right.hash()),
            left: Box::new(left),
            right: Box::new(right)
        }
    }
}
