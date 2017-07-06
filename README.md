# Merkle Simple

Implementation of Merkle Tree in Rust.

## Getting Started

It allows building the Merkle Tree and proof an inclusion of data. This is rather a project I started learning Rust with so don't expect much on it. It uses SHA256 only for the sake of simplicity.

### Prerequisites

In order to use library put this line in your Cargo.toml under the [dependencies] section:

```
[dependencies]
merkle_simple = "0.1.0"
```

The library itself written in a way of having as less dependencies as possible and relies on the "crypto-hash" only.

### Code Examples

In order to put your data sctructure into a tree you should make this structure inplement the Hashable trait.

```Rust
extern crate merkle_simple;

use merkle_simple::{MerkleTree, Hashable};

...

// Prepare data. The Hashable trait is already implemented for String
let data = vec![format!("one"), format!("two"), format!("three"), format!("four")];

// Create Merkle Tree
let tree = MerkleTree::from_vector(data).unwrap();
```

How to get incliusion proof of data in the tree:

```Rust
let data_to_check = format!("two");

let proof = tree.get_proof(data_to_check);
assert!(proof.is_some());
```

And validate the proof:

```Rust
...
// Considering we've got Proof as Option
let result = proof.unwrap().validate(tree.root_hash());
assert!(result);
```

## Authors

* **Sergey Kletsun**

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details