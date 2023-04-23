#[cfg(test)]

use crate::simplicial_complex::simplicial_complex::SimplicialComplex;
#[test]
fn test_full_skeleton_present() {

    let sc_full_skeleton = SimplicialComplex::new_from_vec(vec![(0..7).collect()]);
    let sc_no_full_skeleton = SimplicialComplex::new_from_vec(vec![(0..3).collect(), (1..4).collect()]);
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
    let connected_sc = SimplicialComplex::new_from_vec(vec![(0..5).collect()]);
    let disconnected_sc = SimplicialComplex::new_from_vec(vec![(0..5).collect(), (6..10).collect()]);
    assert_eq!(connected_sc.is_connected(), true);
    assert_eq!(disconnected_sc.is_connected(), false);
}