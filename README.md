# Simplicial Topology
A Rust library for working with simplicial complexes.

## Overview
This library provides tools for constructing and manipulating simplicial complexes in Rust. The main data type is `SimplicialComplex`, which represents a simplicial complex as a collection of facets of various dimensions. This has the benefit of being the most memory efficient representation of a simplicial complex.

The ability to generate multiple different models of random simplicial complexes comes ready out of the box with this library.

## Features
- A simplex is represented as a vector with some additional structure (boundary etc.).
- Construct simplicial complexes from a vector of simplexes (or a vector of vectors).
- Compute boundary matrices and betti numbers
- Generate random simplicial complexes. The following models are currently included [Linial-Meshulam](https://link.springer.com/article/10.1007/s00493-006-0027-9), [Lower](https://link.springer.com/chapter/10.1007/978-3-319-31580-5_6), [Upper](https://www.worldscientific.com/doi/10.1142/S1793525320500387) and [Pure](https://arxiv.org/pdf/1806.04566.pdf).

## Usage

To use `simplicial-topology` in your Rust project, add the following to your `Cargo.toml`:

```toml
[dependencies]
simplicial_topology = { git = "https://github.com/lm7272/simplicial-topology.git" }
```

```rust
use simplicial_topology::SimplicialComplex;

let sc = SimplicialComplex::new_from_vec(vec![vec![0, 1], vec![1, 2], vec![1, 2, 3], vec![3, 4], vec![1, 3, 4]]);

println!("The complex has {} facets", sc.facets.len()); // This will output "The complex has 3 facets."

```
