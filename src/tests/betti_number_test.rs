#[cfg(test)]

use crate::simplicial_complex::{simplex::{Facet, Simplex}, simplicial_complex::SimplicialComplex, hypergraph::Hypergraph};

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
    let sc = hg.downward_closure();
    assert_eq!(sc.betti_numbers(), vec![2,1,0])
}