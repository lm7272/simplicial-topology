use crate::simplicial_complex::simplicial_complex::SimplicialComplex;
use crate::utils::utils::{filter_maximal_sets, filter_downward_closed_sets};

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

    pub fn upward_closure(self) -> SimplicialComplex{
        let v: Vec<Vec<usize>> = self.vertices.into_iter().map(|x| vec![x]).collect();
        let subsets = v.into_iter().chain(self.hyperedges).collect::<Vec<Vec<usize>>>();
        SimplicialComplex::new_from_vec(filter_maximal_sets(subsets))
    }

    pub fn downward_closure(self) -> SimplicialComplex{
        let v: Vec<Vec<usize>> = self.vertices.into_iter().map(|x| vec![x]).collect();
        let subsets = v.into_iter().chain(self.hyperedges).collect::<Vec<Vec<usize>>>();
        SimplicialComplex::new_from_vec(filter_downward_closed_sets(subsets))
    }
}