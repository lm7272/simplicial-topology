use crate::utils::utils::{randomly_select_items_from_vec, subvectors};

use crate::simplicial_complex::hypergraph::Hypergraph;

pub fn generate_random_hypergraph(num_vertices: usize, prob_vec: Vec<f64>) -> Hypergraph{
    let possible_vertices: Vec<usize> = (0..num_vertices).collect();
    let vertices: Vec<usize> = randomly_select_items_from_vec(&possible_vertices, prob_vec[0]);
    let mut hyperedges: Vec<Vec<usize>> = Vec::new();
    for k in 1..prob_vec.len(){
        let k_hyperedges: Vec<Vec<usize>> = subvectors(&possible_vertices, k+1);
        hyperedges.append(&mut randomly_select_items_from_vec(&k_hyperedges, prob_vec[k]));
    }
    Hypergraph {
        vertices: vertices,
        hyperedges: hyperedges
    }
}