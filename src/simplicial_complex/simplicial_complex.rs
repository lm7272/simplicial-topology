use nalgebra::DMatrix;
use itertools::Itertools;
use std::collections::HashSet;

use crate::utils::utils::alternating_sum;
use crate::utils::linear_algebra::{rank_smith_normal_matrix, row_nullity_smith_normal_matrix, gaussian_elimination};
use crate::simplicial_complex::simplex::{Simplex, Facet};

pub struct SimplicialComplex {
    facets: Vec<Facet>,
}
impl Clone for SimplicialComplex{
    fn clone(&self) -> Self {
        Self { facets: self.facets.clone() }
    }
}

impl SimplicialComplex {
    pub fn new(facets: Vec<Facet>) -> Self {
        Self {facets}
        //Self { facets: facets.iter().map(|facet| facet.sort()).collect() }
    }

    pub fn new_from_vec(v: Vec<Vec<usize>>) -> Self{
        Self{ facets: v.into_iter().map(|x| Simplex::new(x)).collect() }
    }

    pub fn print(&self) {
        println!("Simplicial Complex has dimension {}. The facets are:", self.dimension());
        for facet in &self.facets {
            facet.print();
        }
    }

    pub fn dimension(&self) -> isize{
        self.facets.iter().map(|v| v.dimension()).max().unwrap()
    }
    pub fn k_skeleton(self, dim: usize) -> Self{
        Self {facets: self.k_faces(dim)}
    }
    pub fn k_faces(&self, dim: usize) -> Vec<Facet>{
        if self.dimension() < 0 {
            return Vec::new()
        }
        if dim > self.dimension().try_into().unwrap(){
            return Vec::new()
        }
        let mut k_faces: Vec<Facet> = vec![];
        for facet in &self.facets {
            k_faces.extend(facet.vertices.iter()
            .combinations(dim+1).map(|c| Facet::new(c.into_iter().copied().collect_vec())))
        }
        let k_faces_set: HashSet<Facet> = k_faces.into_iter().collect();
        k_faces_set.into_iter().collect()
    }

    pub fn euler_characteristic(&self) -> i32{
        if self.dimension() < 0{
            panic!("Empty simplicial complex, Euler characteristic undefined.")
        }
        let dim: usize = self.dimension() as usize;
        let face_count: Vec<i32> = (0..(dim + 1)).map(|x| self.k_faces(x).len() as i32).collect();
        alternating_sum(&face_count)
    }

    pub fn compute_k_boundary_matrix(&self, dim:usize) -> DMatrix<i32> {
        println!("Computing {}-dimensional boundary matrix", dim);
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

    pub fn star(self, simplex: &Facet) -> Self{
        assert!(simplex.dimension().ge(&0), "Must provided a non-empty simplex");
        let k_faces = self.k_faces(simplex.dimension() as usize);
        assert!(k_faces.contains(simplex), "Cannot take the star of a simplex not in the complex.");
        Self{ facets: self.facets.into_iter().filter(|facet| facet.has_subface(simplex)).collect() }
    }

    pub fn link(self, simplex: &Facet) -> Self{
        Self { facets: self.star(&simplex).facets.into_iter().map(|f| f.link(&simplex)).collect()}
    }

    // fn contains_k_skeleton(self, dim: usize){
    //     self.k_faces(dim).len()
    // }

    fn is_connected(self) -> bool{
        self.kth_betti_number(0) == 1
    }

    pub fn kth_betti_number(self, dim: usize) -> i32 {
        let m1 = gaussian_elimination(self.compute_k_boundary_matrix(dim));
        let m2 = gaussian_elimination(self.compute_k_boundary_matrix(dim+1));
        row_nullity_smith_normal_matrix(&m2) - rank_smith_normal_matrix(&m1)
    }
    pub fn betti_numbers(&self) -> Vec<i32>{
        if self.dimension() < 0{
            return vec![]
        }
        let dim = self.dimension() as usize;
        let reduced_bdy_matrices: Vec<DMatrix<i32>> = (1..dim+1).map(|x| gaussian_elimination(self.compute_k_boundary_matrix(x))).collect_vec();
        let mut betti_numbers: Vec<i32> = vec![row_nullity_smith_normal_matrix(&reduced_bdy_matrices[0])];
        let tmp_betti_numbers: &mut Vec<i32> = &mut (0..dim-1).map(|x| row_nullity_smith_normal_matrix(&reduced_bdy_matrices[x+1]) - rank_smith_normal_matrix(&reduced_bdy_matrices[x])).collect_vec();
        betti_numbers.append(tmp_betti_numbers);
        betti_numbers.push((-1i32).pow(dim as u32)*(self.euler_characteristic() - alternating_sum(&betti_numbers)));
        betti_numbers

    }
}