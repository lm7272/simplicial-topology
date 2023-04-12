mod simplicial_complex{
    pub mod simplicial_complex;
}
mod linear_algebra{
    pub mod smith_normal_form;
}
use simplicial_complex::simplicial_complex::{Facet, Simplex, SimplicialComplex};
use linear_algebra::smith_normal_form::{gaussian_elimination};

fn main() {
    let facets: Vec<Facet> = vec![Simplex::new((0..4).collect())];
    let bdy: Vec<Facet> = facets[0].boundary();
    let sc:SimplicialComplex = SimplicialComplex::new(bdy);
    sc.print();
    let dim = 2;
    let m = sc.compute_k_boundary_matrix(dim);
    println!("The {}-dimensional boundary operator is:{}", dim, m);
    //println!("The pivoted boundary operator is:{}", pivot_rows_and_cols(&m, 0));
    println!("The gaussian eliminated boundary operator is:{}", gaussian_elimination(m));
    println!("This simplicial complex has Euler characteristic: {}", sc.euler_characteristic());
}