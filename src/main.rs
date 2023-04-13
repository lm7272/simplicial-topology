mod simplicial_complex{
    pub mod simplicial_complex;
}
mod linear_algebra{
    pub mod smith_normal_form;
}
use simplicial_complex::simplicial_complex::{Facet, Simplex, SimplicialComplex};
use linear_algebra::smith_normal_form::{gaussian_elimination, rank, row_nullity};

fn main() {
    let facets: Vec<Facet> = vec![Simplex::new((0..4).collect())];
    let bdy: Vec<Facet> = facets[0].boundary();
    let sc:SimplicialComplex = SimplicialComplex::new(bdy);
    sc.print(); 
    let dim = 2;
    let m1 = sc.compute_k_boundary_matrix(dim);
    let m2 = sc.compute_k_boundary_matrix(dim+1);
    println!("The {}-dimensional boundary operator is:{}", dim, m1);
    println!("The {}-dimensional boundary operator is:{}", dim+1, m2);
    println!("The {}-Betti number is: {}", dim, row_nullity(m2) - rank(m1));
    println!("This simplicial complex has Euler characteristic: {}", sc.euler_characteristic());
}