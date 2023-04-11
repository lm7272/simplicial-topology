use nalgebra::{DMatrix};
mod simplicial_complex{
    pub mod simplicial_complex;
}
use simplicial_complex::simplicial_complex::{Facet, Simplex, SimplicialComplex};

fn compute_boundary_matrix(k_simplices: &Vec<Facet>, k_plus_one_simplices: &Vec<Facet>) -> DMatrix<i32> {
    let m = k_simplices.len();
    let n = k_plus_one_simplices.len();
    let mut matrix = DMatrix::from_element(m, n, 0);
    // Populate the matrix based on whether k-simplices are in the boundary of a k+1 simplex
    for (i, facet) in k_plus_one_simplices.iter().enumerate() {
        let bdy = facet.boundary();
        for simplex in bdy {
            let index = k_simplices.iter().position(|x| simplex == x);
            if let Some(j) = index {
                matrix[(j, i)] = 1
            }
        }
    }

    matrix
}

fn main() {
    let facets: Vec<Facet> = vec![Simplex::new(vec![0,1]), Simplex::new(vec![1,2]), Simplex::new(vec![0,2])];
    let bdy: Vec<Facet> = vec![Simplex::new(vec![0]), Simplex::new(vec![1]), Simplex::new(vec![2])];
    let m = compute_boundary_matrix(&bdy, &facets);
    println!("{}", m);
}

// fn main() {
//     let simplex: Facet = Simplex::new(vec![0,1,2,3,4]);
//     simplex.print()
// }