use crate::simplicial_complex::simplicial_complex::SimplicialComplex;
use crate::utils::utils::{filter_maximal_sets, par_filter_downward_closed_sets, filter_downward_closed_sets};

pub struct Hypergraph {
    pub vertices: Vec<usize>,
    pub hyperedges: Vec<Vec<usize>>
}

impl Clone for Hypergraph {
    fn clone(&self) -> Self {
        Self { vertices: self.vertices.clone(), hyperedges: self.hyperedges.clone()}
    }
}

impl Hypergraph{

    fn _combine_vertices_and_edges(self) -> Vec<Vec<usize>>{
        let v: Vec<Vec<usize>> = self.vertices.into_iter().map(|x| vec![x]).collect();
        v.into_iter().chain(self.hyperedges).collect::<Vec<Vec<usize>>>()
    }

    pub fn upward_closure(self) -> SimplicialComplex{
        let subsets = self._combine_vertices_and_edges();
        SimplicialComplex::new_from_vec(filter_maximal_sets(subsets))
    }

    pub fn downward_closure(self) -> SimplicialComplex{
        let subsets = self._combine_vertices_and_edges();
        SimplicialComplex::new_from_vec(filter_downward_closed_sets(subsets))
    }

    pub fn par_downward_closure(self) -> SimplicialComplex{
        let subsets = self._combine_vertices_and_edges();
        SimplicialComplex::new_from_vec(par_filter_downward_closed_sets(subsets))
    }
}