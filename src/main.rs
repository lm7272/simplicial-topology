use itertools::Itertools;
use simplicial_topology::simplicial_complex::simplicial_complex::SimplicialComplex;

fn main() {
    let sc:SimplicialComplex = SimplicialComplex::new_from_vec(vec![(0..8).collect_vec()]).k_skeleton(5);
    sc.print(); 
    println!("Betti vector is: {:?}", sc.betti_numbers());
    println!("This simplicial complex has Euler characteristic: {}", sc.euler_characteristic());
}