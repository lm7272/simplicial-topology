use itertools::Itertools;
use crate::{sc, simplex};
use crate::simplicial_complex::simplex::{Simplex, Facet};
use crate::simplicial_complex::simplicial_complex::SimplicialComplex;

//TODO test properly homology for a large complex

#[test]
fn test_full_skeleton_present() {
    let sc_full_skeleton = sc!((1..8).collect());
    let sc_no_full_skeleton = sc![(0..3).collect(), (1..4).collect()];
    for k in 0..7{
        assert_eq!(sc_full_skeleton.contains_full_k_skeleton(k), true);
    }
    assert_eq!(sc_full_skeleton.contains_full_k_skeleton(sc_full_skeleton.dimension() as usize + 1), false);

    for k in 1..4{
        assert_eq!(sc_no_full_skeleton.contains_full_k_skeleton(k), false);
    }
}

#[test]
fn test_is_connected(){
    let connected_sc = sc![(0..5).collect_vec()];
    let disconnected_sc = sc![(0..5).collect_vec(), (6..10).collect_vec()];
    assert_eq!(connected_sc.is_connected(), true);
    assert_eq!(disconnected_sc.is_connected(), false);
}

#[test]
fn test_k_skeleton(){
    let v = (0..10).collect_vec();
    let sc: SimplicialComplex = sc![v.clone()];
    assert_eq!(sc.clone().k_skeleton(9), sc![v.clone()]);
    assert_ne!(sc.k_skeleton(8), sc![v.clone()]);
}

#[test]
fn test_subcomplex(){
    let sc = sc![(0..10).collect_vec()];
    let sc2 = sc![(2..7).collect_vec()];
    assert_eq!(sc.contains(&sc2), true);
    assert_eq!(sc2.contains(&sc), false);
}

#[test]
fn test_add_simplex(){
    let simplex: Facet = simplex![0,1,2,3,4,5,6,7,8,9];
    let mut sc = SimplicialComplex::new(simplex.boundary());
    let sc2 = SimplicialComplex::new(vec![simplex.clone()]);
    sc.add_simplex(simplex);
    assert_eq!(sc, sc2);
}

#[test]
fn test_empty_complex(){
    let mut sc = SimplicialComplex::new_from_vec(vec![]);
    assert_eq!(sc.dimension(), -1);
    let sigma: Facet = simplex!(1);
    sc.add_simplex(sigma);
    assert_eq!(sc, sc![vec![1]]);
}

#[test]
fn test_union_and_intersection(){
    let sc1 = sc![vec![1,2,3]];
    let sc2 = sc![vec![3,4,5]];
    assert_eq!(sc1.union(&sc2), sc![vec![1,2,3], vec![3,4,5]]);
    assert_eq!(sc1.intersection(&sc2), sc![vec![3]]);
}