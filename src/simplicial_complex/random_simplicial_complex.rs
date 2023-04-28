use rayon::prelude::*;

use crate::utils::utils::{randomly_select_items_from_vec, get_subvectors};
use crate::simplicial_complex::hypergraph::Hypergraph;
use super::simplicial_complex::SimplicialComplex;

pub enum Model {
    Lower {num_vertices: usize, prob_vec: Vec<f64>},
    Upper {num_vertices: usize, prob_vec: Vec<f64>},
    LinialMeshulam {num_vertices: usize, dimension: usize, prob: f64},
    Pure {num_vertices: usize, dimension: usize, prob: f64, include_all_vertices: bool}
}

pub fn generate_random_hypergraph(num_vertices: usize, prob_vec: Vec<f64>) -> Hypergraph{
    let possible_vertices: Vec<usize> = (0..num_vertices).collect();
    let vertices: Vec<usize> = randomly_select_items_from_vec(&possible_vertices, prob_vec[0]);
    let mut hyperedges: Vec<Vec<usize>> = Vec::new();
    for k in 1..prob_vec.len(){
        let k_hyperedges: Vec<Vec<usize>> = get_subvectors(&possible_vertices, k+1);
        hyperedges.extend(randomly_select_items_from_vec(&k_hyperedges, prob_vec[k]));
    }
    Hypergraph {
        vertices,
        hyperedges
    }
}

pub fn par_generate_random_hypergraph(num_vertices: usize, prob_vec: Vec<f64>) -> Hypergraph{
    let possible_vertices: Vec<usize> = (0..num_vertices).collect();
    let vertices: Vec<usize> = randomly_select_items_from_vec(&possible_vertices, prob_vec[0]);
    let hyperedges: Vec<Vec<usize>> = (1..prob_vec.len()).collect::<Vec<usize>>().par_iter().map(
        |k| {
                        let k_hyperedges: Vec<Vec<usize>> = get_subvectors(&possible_vertices, k+1);
                        randomly_select_items_from_vec(&k_hyperedges, prob_vec[*k])
                }
            ).collect::<Vec<Vec<Vec<usize>>>>()
            .into_iter()
            .flat_map(|inner_vec| inner_vec.into_iter())
            .collect();
    Hypergraph {
        vertices,
        hyperedges
    }
}

pub fn generate_random_simplicial_complex(model: Model) -> SimplicialComplex{

    let sc = match model {
        Model::LinialMeshulam { num_vertices, dimension, prob } => {
            let mut prob_vec: Vec<f64> = vec![0; dimension - 1].into_iter().map(|x| x as f64).collect::<Vec<f64>>();
            prob_vec.push(1.0);
            prob_vec.push(prob);
            generate_random_hypergraph(num_vertices, prob_vec).upward_closure()
        },
        Model::Lower { num_vertices, prob_vec } => {
            generate_random_hypergraph(num_vertices, prob_vec).par_downward_closure()
        },
        Model::Upper { num_vertices, prob_vec } => {
            generate_random_hypergraph(num_vertices, prob_vec).upward_closure()
        },
        Model::Pure { num_vertices, dimension, prob, include_all_vertices } => {
            let mut prob_vec: Vec<f64> = Vec::new();
            if include_all_vertices {
                prob_vec.push(1.0)
            }
            else {
                prob_vec.push(0.0)
            }
            prob_vec.extend(vec![0.0; dimension - 1]);
            prob_vec.push(prob);
            generate_random_hypergraph(num_vertices, prob_vec).upward_closure()
        }

    };
    sc
}