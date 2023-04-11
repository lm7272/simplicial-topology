mod simplicial_complex{
    pub mod simplicial_complex;
}
mod linear_algebra{
    pub mod smith_normal_form;
}
use simplicial_complex::simplicial_complex::{Facet, Simplex, SimplicialComplex};
use linear_algebra::smith_normal_form::pivot_rows_and_cols;

fn main() {
    let facets: Vec<Facet> = vec![Simplex::new((0..4).collect())];
    let bdy: Vec<Facet> = facets[0].boundary();
    let sc:SimplicialComplex = SimplicialComplex::new(bdy);
    sc.print();
    let dim = 2;
    let faces = sc.k_faces(dim);
    for face in faces{
        face.print();
    }
    let m = sc.compute_k_boundary_matrix(dim);
    println!("The {}-dimensional boundary operator is:{}", dim, m);
    println!("The {}-dimensional boundary operator is:{}", dim, pivot_rows_and_cols(&m, 0));
    println!("This simplicial complex has Euler characteristic: {}", sc.euler_characteristic());
}