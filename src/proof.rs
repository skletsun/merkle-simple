use treeelement::TreeElement;
use utils::{Hashable, Hasher};

#[derive(Debug)]
/// Implements verification for inclusion of data into treeEleTreeElement
pub struct Proof<T> {
    /// Value to verify
    value: T,

    /// Root hash of the treeEleTreeElement
    root_hash: Vec<u8>,

    /// Starting point of proof-path
    path: PathItem,
}

impl<T> Proof<T> {
    /// Produces new Proof structure
    pub fn new(value: T, root_hash: Vec<u8>, path_item: PathItem) -> Self {
        Proof {
            value: value,
            root_hash: root_hash,
            path: path_item,
        }
    }

    /// Validates the path
    pub fn validate(&self, root_hash: &[u8]) -> bool {
        // Check for the obvious things first
        if root_hash != self.root_hash.as_slice() || self.path.hash != root_hash {
            return false;
        }

        // recursive check the path
        self.validate_recursive(&self.path)
    }

    fn validate_recursive(&self, path_item: &PathItem) -> bool {
        match path_item.sub_item {
            Some(ref child) => {
                match path_item.sibling_hash {
                    Some(Position::Left(ref hash)) => {
                        // calculating node hash taking into account that sibling's hash should be
                        // the first parameter of hash_node_data() since it is positioned left
                        let calculated_hash = Hasher::hash_node_data(hash, &child.hash);
                        // it should match the node's hash
                        (calculated_hash == path_item.hash) && self.validate_recursive(child)
                    }
                    Some(Position::Right(ref hash)) => {
                        // calculating node hash taking into account that sibling's hash should be
                        // the second parameter of hash_node_data() since it is positioned right
                        let calculated_hash = Hasher::hash_node_data(&child.hash, hash);
                        // it should match the node's hash
                        (calculated_hash == path_item.hash) && self.validate_recursive(child)
                    }
                    None => false,
                }
            }
            None => path_item.sibling_hash.is_none(),
        }
    }
}

#[derive(Debug, Clone)]
/// Data structure that the inclusion proof path consist of. Holds the information about
/// (1) hash values of this and sibling nodes and (2) a reference to the sub-item.
pub struct PathItem {
    /// Hash of current node
    hash: Vec<u8>,

    /// Hash of sibling if any. Leaf element doesn't have any siblings so we wrap it with Option.
    sibling_hash: Option<Position<Vec<u8>>>,

    /// Reference to the child node in the direction from root to leafs
    sub_item: Option<Box<PathItem>>,
}

impl PathItem {
    /// Recursively creates path according to the exact type of nodes until element is found.
    /// Returns None in case element hasn't been found in the tree.
    pub fn create_path<T: Hashable>(
        node: &TreeElement<T>,
        hash_to_find: &[u8],
    ) -> Option<PathItem> {
        match *node {
            TreeElement::Node {
                ref left,
                ref right,
                ref hash,
            } => PathItem::new_node_proof(hash, hash_to_find, left, right),
            TreeElement::Leaf { ref hash, .. } => PathItem::new_leaf_proof(hash, hash_to_find),
        }
    }

    /// Creates an item in the inclusion proof path for a leaf type of node
    fn new_leaf_proof(hash: &[u8], hash_to_find: &[u8]) -> Option<PathItem> {
        if *hash == *hash_to_find {
            Some(PathItem {
                hash: hash.to_vec(),
                sibling_hash: None,
                sub_item: None,
            })
        } else {
            None
        }
    }

    /// Creates an item in the inclusion proof path for an internal node (not leaf)
    fn new_node_proof<T: Hashable>(
        hash: &[u8],
        hash_to_find: &[u8],
        left: &TreeElement<T>,
        right: &TreeElement<T>,
    ) -> Option<PathItem> {
    // Recusively go to the left node
    PathItem::create_path(left, hash_to_find)
        .map(|item| {
            (item, Some(Position::Right(right.hash().clone().to_vec())))
        }).or_else(|| {
            let child_item = PathItem::create_path(right, hash_to_find);
            child_item.map(|item| {
                (item, Some(Position::Left(left.hash().clone().to_vec())))
            })
        }).map(|(path_item, sibl_hash)| {
            // And finally construct the path item
            PathItem {
                hash: hash.to_vec(),
                sibling_hash: sibl_hash,
                sub_item: Some(Box::new(path_item)),
            }
        })
    }
}

#[derive(Debug, Clone)]
/// Encapsulates positioning of sibling node in the inclusion path
enum Position<T> {
    /// For the left sibling
    Left(T),

    /// For the right sibling
    Right(T),
}
