use TreeElement;
use utils::{Hashable, Hasher};

#[derive(Debug)]
/// Implements verification for inclusion of data into tree
pub struct Proof<T> {

    /// Value to verify
    value: T,

    /// Root hash of the tree
    root_hash: Vec<u8>,

    /// Starting point of proof-path
    path: PathItem
}

#[derive(Debug, Clone)]
/// Data structure that the inclusion proof path consist of. Holds the information about
/// (1) hash values of this and sibling nodes and (2) a reference to the sub-item.
struct PathItem {

    /// Hash of current node
    hash: Vec<u8>,

    /// Hash of sibling if any. Leaf element doesn't have any siblings so we wrap it with Option.
    sibling_hash: Option<Position<Vec<u8>>>,

    /// Reference to the child node in the direction from root to leafs
    sub_item: Option<Box<PathItem>>
}

#[derive(Debug, Clone)]
/// Encapsulates positioning of sibling node in the inclusion path
enum Position<T> {

    /// For the left sibling
    Left(T),

    /// For the right sibling
    Right(T),
}