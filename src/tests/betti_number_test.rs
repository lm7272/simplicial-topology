#[cfg(test)]

use crate::simplicial_complex::{simplex::{Facet, Simplex}, simplicial_complex::SimplicialComplex, hypergraph::Hypergraph};
use crate::{sc, simplex};

#[test]
fn test_spheres() {
    for n in 3..10{
        let simplex: Facet = Simplex::new((0..n).collect());
        let sc = SimplicialComplex::new(simplex.boundary());
        for i in 1..n-2{
            assert_eq!(sc.clone().kth_betti_number(i), 0)    
        }
        assert_eq!(sc.kth_betti_number(n-2), 1)
}
}

#[test]
fn test_from_hypergraph_upward() {
    let hg = Hypergraph {
        vertices: vec![1, 2, 3, 4, 5, 6],
        hyperedges: vec![
            vec![1, 2],
            vec![2, 3],
            vec![3, 4],
            vec![1, 4],
            vec![2, 4],
            vec![1, 5],
            vec![1, 2, 3],
            vec![2, 3, 4],
            vec![1, 3, 4],
            vec![1, 2, 4]
        ],
    };
    let sc = hg.upward_closure();
    assert_eq!(sc.betti_numbers(), vec![2,0,1])
}

#[test]
fn test_from_hypergraph_downward() {
    let hg = Hypergraph {
        vertices: vec![1, 2, 3, 4, 5, 6],
        hyperedges: vec![
            vec![1, 2],
            vec![2, 3],
            vec![3, 1],
            vec![1, 4],
            vec![2, 4],
            vec![1, 5],
            vec![4, 1, 3],
            vec![1, 4, 2]
        ],
    };
    let sc = hg.par_downward_closure();
    assert_eq!(sc.betti_numbers(), vec![2,1,0])
}

#[test]
fn test_from_hypergraph_downward2() {
    let hg = Hypergraph {
        vertices: vec![1, 2, 3, 4],
        hyperedges: vec![
            vec![1, 2],
            vec![2, 3],
            vec![1, 3],
            vec![1, 4],
            vec![2, 4],
            vec![3, 4],
            vec![1, 2, 3],
            vec![1, 2, 4],
            vec![1, 3, 4],
            vec![2, 3, 4],
            vec![1, 2, 3, 4],
        ],
    };
    let sc = hg.par_downward_closure();
    let sc_golden = sc![vec![2,1,3,4]];
    assert_eq!(sc, sc_golden)
}
#[test]
fn test_alexander_duality(){
    let sigma: Facet = simplex![1,2,3];
    let tau: Facet = simplex![1,4,5];
    let mut sc: SimplicialComplex = sigma.boundary_as_complex().union(&tau.boundary_as_complex());
    sc.add_simplex(simplex![1,2,3]);
    sc.add_simplex(simplex![6]);
    //sc.betti_numbers() = (2,1,0) reduced this becomes (1,1,0)
    //dual map b_i(X)->b_{3-i}(X*) gives (0,0,1,1) which unreduced is (1,0,1,1)
    assert_eq!(sc.alexander_dual().betti_numbers(), vec![1,0,1,1])
}