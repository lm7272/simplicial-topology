use std::{hash::{Hash, Hasher}, collections::HashSet};
use itertools::Itertools;
pub trait Simplex {
    type Boundary: Sized;
    fn new(vertices: Vec<usize>) -> Self;
    fn dimension(&self) -> isize;
    fn boundary(&self) -> Self::Boundary;
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
        let mut result = Vec::with_capacity(n.try_into().unwrap());
        for i in 0..n {
            // create a new simplex with one less vertex
            let vertices = self.vertices.iter().enumerate()
                .filter(|&(j, _)| j != i.try_into().unwrap())
                .map(|(_, v)| *v)
                .collect();
            result.push(Facet::new(vertices));
        }
        result
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

