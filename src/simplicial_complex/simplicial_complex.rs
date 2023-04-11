use nalgebra::DMatrix;
use itertools::Itertools;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
pub trait Simplex {
    type Boundary: Sized;
    fn new(vertices: Vec<usize>) -> Self;
    fn facet(&mut self, vertices: Vec<usize>);
    fn dimension(&self) -> usize;
    fn boundary(&self) -> Self::Boundary;
    fn print(&self);
}

#[derive(Debug)]
pub struct Facet{
    vertices: Vec<usize>
}
pub struct SimplicialComplex {
    facets: Vec<Facet>,
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

    fn facet(&mut self, vertices: Vec<usize>) {
        self.vertices = vertices;
    }

    fn dimension(&self) -> usize{
        self.vertices.len() - 1
    }

    fn boundary(&self) -> Vec<Self> {
        let n: usize = self.dimension() + 1;
        let mut result = Vec::with_capacity(n);
        for i in 0..n {
            // create a new simplex with one less vertex
            let vertices = self.vertices.iter().enumerate()
                .filter(|&(j, _)| j != i)
                .map(|(_, v)| *v)
                .collect();
            result.push(Facet::new(vertices));
        }
        result
    }

    fn print(&self) {
        println!("{:?}", self.vertices);
    }
}

impl SimplicialComplex {
    pub fn new(facets: Vec<Facet>) -> Self {
        Self { facets }
    }

    pub fn print(&self) {
        println!("Simplicial Complex has dimension {}. The facets are:", self.dimension());
        for facet in &self.facets {
            facet.print();
        }
    }

    pub fn dimension(&self) -> usize{
        self.facets.iter().map(|v| v.dimension()).max().unwrap()
    }

    pub fn k_faces(&self, dim: usize) -> Vec<Facet>{
        let mut k_faces: Vec<Facet> = vec![];
        for facet in &self.facets {
            k_faces.extend(facet.vertices.iter()
            .combinations(dim+1).map(|c| Facet::new(c.into_iter().copied().collect_vec())))
        }
        let k_faces_set: HashSet<Facet> = k_faces.into_iter().collect();
        k_faces_set.into_iter().collect()
    }

    pub fn euler_characteristic(&self) -> i32{
        let mut chi: i32 = 0;
        for i in 0..(self.dimension()+1){
            chi += (-1 as i32).pow(i as u32) * self.k_faces(i).len() as i32;
        }
        chi
    }

    pub fn compute_k_boundary_matrix(&self, dim:usize) -> DMatrix<i32> {
        let k_minus_one_simplices: Vec<Facet> = self.k_faces(dim-1);
        let k_simplices: Vec<Facet> = self.k_faces(dim);
        let mut bdy_matrix = DMatrix::from_element(k_minus_one_simplices.len(), k_simplices.len(), 0);
        // Populate the matrix based on whether k-simplices are in the boundary of a k+1 simplex
        for (i, facet) in k_simplices.iter().enumerate() {
            let bdy = facet.boundary();
            for simplex in bdy {
                let index = k_minus_one_simplices.iter().position(|x| simplex == x);
                if let Some(j) = index {
                    bdy_matrix[(j, i)] = 1
                }
            }
        }
    
        bdy_matrix
    }

    pub fn exchange_rows_and_cols(&self, a: &DMatrix<i32>, x: usize) -> DMatrix<i32>{
        let (k, l) = match (x..a.nrows()).flat_map(|i| (x..a.ncols()).map(move |j| (i, j))).find(|&(i, j)| a[(i, j)] == 1) {
            Some((i, j)) => (i, j),
            None => return a.clone(),
        };
        let mut b = a.clone();
        println!("{}, {}", k, l);
        b.swap_rows(x, k);
        b.swap_columns(x, l);
        b
    }    
}