mod simplicial_complex{
    pub mod simplicial_complex;
}
use simplicial_complex::simplicial_complex::{Facet, Simplex, SimplicialComplex};

fn main() {
    let facets: Vec<Facet> = vec![Simplex::new((0..3).collect()), Simplex::new((0..4).collect())];
    let sc:SimplicialComplex = SimplicialComplex::new(facets);
    sc.print();
    let dim = 2;
    let faces = sc.k_faces(dim);
    for face in faces{
        face.print();
    }
    let m = sc.compute_k_boundary_matrix(dim);
    println!("The {}-dimensional boundary operator is:{}", dim, m);
    println!("This simplicial complex has Euler characteristic: {}", sc.euler_characteristic());
}