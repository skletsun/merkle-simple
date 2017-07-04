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

extern crate crypto_hash;

mod merkletree;
pub use merkletree::MerkleTree;

mod treeelement;
pub use treeelement::TreeElement;

mod utils;
pub use utils::{Hashable, Hasher};

#[cfg(test)]
mod tests;
