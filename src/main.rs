use simplicial_topology::simplicial_complex::simplex::{Facet,Simplex};
use simplicial_topology::simplicial_complex::simplicial_complex::SimplicialComplex;
use simplicial_topology::utils::linear_algebra::{gaussian_elimination, rank_smith_normal_matrix, row_nullity_smith_normal_matrix};

fn main() {
    let facets: Vec<Facet> = vec![Simplex::new((0..8).collect())];
    //let bdy: Vec<Facet> = facets[0].boundary();
    let sc:SimplicialComplex = SimplicialComplex::new(facets).k_skeleton(5);
    sc.print(); 
    // println!("The {}-dimensional boundary operator is:{}", dim, m1);
    // println!("The {}-dimensional boundary operator is:{}", dim+1, m2);
    println!("Betti vector is: {:?}", sc.betti_numbers());
    println!("This simplicial complex has Euler characteristic: {}", sc.euler_characteristic());
}