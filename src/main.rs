mod simplicial_complex{
    pub mod simplicial_complex;
}
use simplicial_complex::simplicial_complex::{Facet, Simplex, SimplicialComplex};

fn main() {
    let facets: Vec<Facet> = vec![Simplex::new((0..6).collect())];
    let sc:SimplicialComplex = SimplicialComplex::new(facets);
    sc.print();
    let dim = 3;
    let faces = sc.k_faces(dim);
    for face in faces{
        face.print();
    }
    let m = sc.compute_k_boundary_matrix(dim);
    println!("{}", m);
}

// fn main() {
//     let simplex: Facet = Simplex::new(vec![0,1,2,3,4]);
//     simplex.print()
// }