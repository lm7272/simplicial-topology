use itertools::Itertools;
use simplicial_topology::simplicial_complex::simplex::Simplex;
use simplicial_topology::simplicial_complex::simplicial_complex::SimplicialComplex;
use simplicial_topology::simplicial_complex::hypergraph::{Hypergraph};
use simplicial_topology::simplicial_complex::random_simplicial_complex::generate_random_hypergraph;

fn main() {
    // let hg = Hypergraph {
    //     vertices: vec![1, 2, 3, 4, 6],
    //     hyperedges: vec![
    //         vec![1, 2],
    //         vec![2, 3],
    //         vec![3, 4],
    //         vec![4, 1],
    //         vec![2, 4],
    //         vec![1,5],
    //         vec![1,2,3]
    //     ],
    // };
    std::env::set_var("RUST_BACKTRACE", "1");
    let hg = generate_random_hypergraph(30, vec![1.0,0.5,0.5,1.0]);
    //print!("Vertices: {:?}\nHyperedges: {:?}\n", hg.vertices, hg.hyperedges);
    let sc = hg.clone().downward_closure();
    //sc.print();
    println!("Betti vector is: {:?}", sc.betti_numbers());
    println!("This simplicial complex has Euler characteristic: {}", sc.euler_characteristic());
    let sc2 = hg.upward_closure();
    //sc2.print();
    println!("Betti vector is: {:?}", sc2.betti_numbers());
    println!("This simplicial complex has Euler characteristic: {}", sc2.euler_characteristic());
    //sc.clone().star(&Simplex::new(vec![1])).print();
    //sc.link(&Simplex::new(vec![1])).print();
    // let sc:SimplicialComplex = upward_closure(hg);
    // sc.print();
    // //let sc:SimplicialComplex = SimplicialComplex::new_from_vec(vec![(0..15).collect_vec()]).k_skeleton(5);
    // // sc.print(); 
}