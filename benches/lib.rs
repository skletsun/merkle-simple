#![feature(test)]

extern crate merkle_simple;
extern crate test;

use test::Bencher;
use merkle_simple::{MerkleTree, Hashable};

#[bench]
fn tree_creation(b: &mut Bencher) {
    let input_data = vec!["one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), "five".to_string(), "six".to_string(), "seven".to_string(), "eight".to_string()];

    b.iter(|| MerkleTree::from_vector(input_data.clone()));
}

#[bench]
fn big_tree_creation(b: &mut Bencher) {
    let mut input_data: Vec<String> = Vec::with_capacity(1000);
    let lenght = 2048;
    let mut s: String = String::with_capacity(lenght);
    for _ in 1..lenght {
        s.push('a');
    }

    for _ in 1..10000 {
        input_data.push(s.clone());
    }

    b.iter(|| MerkleTree::from_vector(input_data.clone()));
}