#[cfg(test)]

use crate::simplicial_complex::simplicial_complex::SimplicialComplex;
#[test]
fn test_full_skeleton_present() {

    let sc = SimplicialComplex::new_from_vec(vec![(0..7).collect()]);
    for k in 0..7{
        assert_eq!(sc.contains_full_k_skeleton(k), true);
    }
    assert_eq!(sc.contains_full_k_skeleton(sc.dimension() as usize + 1), false);
}

#[test]
fn test_full_skeleton_not_present() {

    let sc = SimplicialComplex::new_from_vec(vec![(0..3).collect(), (1..4).collect()]);
    for k in 1..4{
        assert_eq!(sc.contains_full_k_skeleton(k), false);
    }
}
