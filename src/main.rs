use simplicial_topology::{sc, simplex};
use simplicial_topology::simplicial_complex::simplex::Simplex;
use simplicial_topology::simplicial_complex::simplicial_complex::SimplicialComplex;
use simplicial_topology::simplicial_complex::hypergraph::{Hypergraph};
use simplicial_topology::simplicial_complex::random_simplicial_complex::{generate_random_hypergraph, generate_random_simplicial_complex, Model};

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    println!("generating random hypergraph");
    // let hg = generate_random_hypergraph(30, vec![1.0,0.7,0.8,0.1]);
    // let sc = generate_random_simplicial_complex(Model::Pure { num_vertices: 30, dimension: 3, prob: 0.1, include_all_vertices: true });
    // println!("Pure random complex has Betti vector is: {:?}", sc.betti_numbers());
    
    // let sc = hg.clone().par_downward_closure();
    // println!("Downward closure Betti vector is: {:?}", sc.betti_numbers());
    // println!("This simplicial complex has Euler characteristic: {}", sc.euler_characteristic());

    // let sc = hg.clone().upward_closure();
    // println!("Upward closure Betti vector is: {:?}", sc.betti_numbers());
    // println!("This simplicial complex has Euler characteristic: {}", sc.euler_characteristic());
    let mut sc = sc![vec![1,2], vec![2,3], vec![1,3], vec![1,4], vec![4,5], vec![1,5]];
    sc.add_simplex(simplex![1,4,5]);
    println!("Betti vector is: {:?}", sc.betti_numbers());
    println!("This simplicial complex has Euler characteristic: {}", sc.euler_characteristic());
}