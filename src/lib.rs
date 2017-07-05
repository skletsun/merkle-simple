#![deny(missing_docs,
        missing_debug_implementations,
        missing_copy_implementations,
        trivial_casts,
        trivial_numeric_casts,
        unsafe_code,
        unstable_features,
        unused_import_braces,
        unused_qualifications
)]

//! Implementation of Merkle Tree in Rust.

extern crate crypto_hash;

mod merkletree;
pub use merkletree::MerkleTree;

mod treeelement;
pub use treeelement::TreeElement;

mod utils;
pub use utils::{Hashable, Hasher};

mod proof;
pub use proof::{PathItem, Proof};

#[cfg(test)]
mod tests;
