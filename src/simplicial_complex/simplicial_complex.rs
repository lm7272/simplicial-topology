use nalgebra::DMatrix;
use itertools::Itertools;
use std::collections::HashSet;
use num_integer::binomial;
use rayon::prelude::*;

use crate::utils::utils::{alternating_sum, filter_maximal_sets};
use crate::utils::linear_algebra::{rank_smith_normal_matrix, row_nullity_smith_normal_matrix, gaussian_elimination};
use crate::simplicial_complex::simplex::{Simplex, Facet};

#[derive(Debug)]
pub struct SimplicialComplex {
    facets: Vec<Facet>,
}

impl PartialEq for SimplicialComplex {
    fn eq(&self, other: &Self) -> bool {
        if self.facets.len() != other.facets.len() {
            return false;
        }
        for (i, facet) in self.facets.iter().enumerate() {
            if !other.facets.contains(facet){
                return false;
            }
        }
        true
    }
}


impl Clone for SimplicialComplex{
    fn clone(&self) -> Self {
        Self { facets: self.facets.clone() }
    }
}

impl SimplicialComplex {
    pub fn new(facets: Vec<Facet>) -> Self {
        Self::new_from_vec(facets.into_iter().map(|facet: Facet| facet.vertices).collect())
    }

    pub fn new_from_vec(v: Vec<Vec<usize>>) -> Self{
        let facets: Vec<Facet> = filter_maximal_sets(v).into_iter().map(|x| Simplex::new(x)).collect();
        Self { facets: facets.into_iter().map(|facet| facet.sort()).collect() }
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
        Self::new((0..(dim+1)).flat_map(|k| self.k_faces(k)).collect())
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
            let vertices = &facet.vertices;
            k_faces.extend(vertices.iter()
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

    fn compute_reduced_k_boundary_matrix(&self, dim: usize) -> DMatrix<i32>{
        gaussian_elimination(self.compute_k_boundary_matrix(dim))
    }

    fn compute_k_boundary_matrix(&self, dim:usize) -> DMatrix<i32> {
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

    pub fn contains_full_k_skeleton(&self, dim: usize) -> bool{
        if (self.dimension() < 0) | (dim > self.dimension() as usize){
            return false
        }
        let num_faces = self.k_faces(dim).len();
        num_faces == binomial(self.k_faces(0).len(), dim +1)
    }

    pub fn is_connected(&self) -> bool{
        self.kth_betti_number(0) == 1
    }

    pub fn kth_betti_number(&self, dim: usize) -> i32 {
        if dim == 0 {
            return row_nullity_smith_normal_matrix(&self.compute_reduced_k_boundary_matrix(1))
        }
        let m1 = &self.compute_reduced_k_boundary_matrix(dim);
        let m2 = &self.compute_reduced_k_boundary_matrix(dim+1);
        row_nullity_smith_normal_matrix(m2) - rank_smith_normal_matrix(m1)
    }
    pub fn betti_numbers(&self) -> Vec<i32>{
        if self.dimension() < 0{
            return vec![]
        }
        let dim = self.dimension() as usize;
        let dimensions: Vec<usize> = (1..(dim+1)).collect();
        let bdy_matrices: Vec<DMatrix<i32>> = dimensions.into_par_iter().map(|x| self.compute_reduced_k_boundary_matrix(x)).collect();
        let mut betti_numbers: Vec<i32> = vec![row_nullity_smith_normal_matrix(&bdy_matrices[0])];
        let tmp_betti_numbers: &mut Vec<i32> = &mut (0..dim-1).map(|x| row_nullity_smith_normal_matrix(&bdy_matrices[x+1]) - rank_smith_normal_matrix(&bdy_matrices[x])).collect_vec();
        betti_numbers.append(tmp_betti_numbers);
        betti_numbers.push((-1i32).pow(dim as u32)*(self.euler_characteristic() - alternating_sum(&betti_numbers)));
        betti_numbers

    }
}