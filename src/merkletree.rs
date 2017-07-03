// use crypto_hash::{Algorithm, digest};
use std::collections::VecDeque;
use utils::Hashable;
use treeelement::TreeElement;

#[derive(Debug)]
pub struct MerkleTree<T> where T: Hashable {
    /// The root element
    root: TreeElement<T>,

    /// The amount of elements
    count: usize,

    /// The height of the tree
    height: usize
}

impl <T> MerkleTree<T> where T: Hashable {

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn root_hash(&self) -> &[u8] {
        self.root.hash()
    }

    pub fn from_vector(data: Vec<T>) -> Option<Self> where T: Hashable {
        let count = data.len();

        match count {
            0 => None,
            _ => {
                let mut height = 0;
                let mut src = VecDeque::with_capacity(count);

                // create first layer of leaves
                for val in data {
                    src.push_back(TreeElement::new_leaf(val));
                }

                // build tree without recursion, layer by layer
                while src.len() > 1 {
                    let mut new_layer: VecDeque<TreeElement<T>> = VecDeque::with_capacity(src.len()/2);

                    while !src.is_empty() {
                        // check for the case when we have the one element only - it will be the Leaf
                        if src.len() == 1 {
                            // It's a leaf - push it to the next turn of processing
                            new_layer.push_back(src.pop_front().unwrap());
                        } else {
                            // we take two elements and produce the Node
                            let left_node = src.pop_front().unwrap();
                            let right_node = src.pop_front().unwrap();
                            let node = TreeElement::new_node(left_node, right_node);
                            // and push it for further processing
                            new_layer.push_back(node);
                        }
                    }
                    // we've just processed the new layer of our tree
                    // increase the height of tree
                    height += 1;
                    // pass our prepared queue to the next iteration if any
                    src = new_layer;
                }
                // we ended up with only one element - this is the root node
                let root = src.pop_back().unwrap();

                // return the resulting tree
                Some(
                    MerkleTree {
                        root: root,
                        count: count,
                        height: height
                    }
                )
            }
        }
    }
}
