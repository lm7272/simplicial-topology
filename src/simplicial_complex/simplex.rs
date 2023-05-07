use std::{hash::{Hash, Hasher}, collections::HashSet};
use itertools::Itertools;

use super::simplicial_complex::SimplicialComplex;
pub trait Simplex {
    type Boundary: Sized;
    fn new(vertices: Vec<usize>) -> Self;
    fn dimension(&self) -> isize;
    fn boundary(&self) -> Self::Boundary;
    fn boundary_as_complex(&self) -> SimplicialComplex;
    fn sort(self) -> Self;
    fn has_subface(&self, simplex: &Facet) -> bool;
    fn link(self, simplex: &Facet) -> Self;
    fn print(&self);
}

#[derive(Debug)]
pub struct Facet{
    pub vertices: Vec<usize>
}

impl PartialEq for Facet {
    fn eq(&self, other: &Self) -> bool {
        self.vertices == other.vertices
    }
}

impl PartialEq<&Facet> for Facet {
    fn eq(&self, other: &&Facet) -> bool {
        self.vertices == other.vertices
    }
}

impl Eq for Facet {}

impl Hash for Facet {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.vertices.hash(state);
    }
}

impl Clone for Facet {
    fn clone(&self) -> Self {
        Self { vertices: self.vertices.clone() }
    }
}

pub fn simplex_join(sigma: &Facet, tau: &Facet) -> Facet{
    let mut v = sigma.vertices.clone();
    let mut w = tau.vertices.clone();
    if v.iter().any(|vertex| w.contains(vertex)){
        panic!("Expected disjoint simplices to join.")
    }
    v.append(&mut w);
    Simplex::new(v)
}

pub fn simplex_intersection(sigma: &Facet, tau: &Facet) -> Facet{
    let mut v = sigma.vertices.clone();
    let w = tau.vertices.clone();
    v.retain(|&vertex| w.contains(&vertex));
    Simplex::new(v)
}

impl Simplex for Facet{
    type Boundary = Vec<Self>;

    fn new(vertices: Vec<usize>) -> Self{ 
        Facet {vertices}
    }

    fn sort(self) -> Self {
        Facet { vertices: self.vertices.into_iter().sorted().collect()}
    }

    fn dimension(&self) -> isize{
        if self.vertices.is_empty(){
            return -1
        }
        (self.vertices.len() - 1) as isize
    }

    fn boundary(&self) -> Vec<Self> {
        let n: isize = self.dimension() + 1;
        let mut result: Vec<Facet> = Vec::new();
        for i in 0..n {
            // create a new simplex with one less vertex
            let vertices = self.vertices.iter().enumerate()
                .filter(|&(j, _)| j != <isize as TryInto<usize>>::try_into(i).unwrap())
                .map(|(_, v)| *v)
                .collect();
            result.push(Facet::new(vertices));
        }
        result
    }

    fn boundary_as_complex(&self) -> SimplicialComplex {
        SimplicialComplex::new(self.boundary())
    }

    fn has_subface(&self, simplex: &Facet) -> bool {
        let simplex_set = simplex.vertices.iter().cloned().collect::<HashSet<usize>>();
        let facet_set = self.vertices.iter().cloned().collect::<HashSet<usize>>();
        simplex_set.is_subset(&facet_set)
    }

    fn link(self, simplex: &Facet) -> Self {
        Self { vertices:  self.vertices.iter().filter(|&elem| !simplex.vertices.contains(elem)).cloned().collect()}
    }

    fn print(&self) {
        println!("{:?}", self.vertices);
    }
}

