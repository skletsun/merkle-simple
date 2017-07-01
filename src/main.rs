extern crate crypto_hash;

use crypto_hash::{Algorithm, digest};
use std::collections::VecDeque;

pub trait Hashable {
    fn get_bytes(&self) -> &[u8];
}

impl Hashable for String {
     fn get_bytes(&self) -> &[u8] {
         &self.as_bytes()
     }
}

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

    fn new_leaf(value: T) -> TreeElement <T> {
        TreeElement::Leaf {
            hash: digest(Algorithm::SHA256, value.get_bytes()),
            value: value
        }
    }

    fn new_node(left: TreeElement<T>, right: TreeElement<T>) -> TreeElement <T> {
        let mut buffer: Vec<u8> = Vec::with_capacity(left.hash().len() + right.hash().len());
        buffer.extend(left.hash());
        buffer.extend(right.hash());

        TreeElement::Node {
            hash: digest(Algorithm::SHA256, &buffer),
            left: Box::new(left),
            right: Box::new(right)
        }
    }
}

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

    fn from_vector(data: Vec<T>) -> Self where T: Hashable {
        // TODO Handle empty array case

        let count = data.len();
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
        let root = src.pop_back().expect("Error retrieving the root node");

        // return the resulting tree
        MerkleTree {
            root: root,
            count: count,
            height: height
        }
    }
}

#[derive(Debug)]
struct Transaction {
    data: Vec<u8>
}

impl Transaction {
    fn new(data: Vec<u8>) -> Self {
        Transaction {
            data: data
        }
    }
}

impl Hashable for Transaction {
     fn get_bytes(&self) -> &[u8] {
         &self.data
     }
}

fn main() {

    let leaf_left = TreeElement::new_leaf(format!("left node"));
    let leaf_right = TreeElement::new_leaf(format!("right node"));

    let node = TreeElement::new_node(leaf_left, leaf_right);

    println!("Element: {:?}", node);

    let tree = MerkleTree::from_vector(vec![Transaction::new(vec![42; 256]), Transaction::new(vec![43; 128])]);
    // let tree = MerkleTree::from_vector(vec![format!("abc"), format!("def"), format!("jsadlgkjsdlkg"), format!("4335fhdf87y54"), format!("the fifth element!")]);
    println!("Merkle Tree: {:?}", tree);
}