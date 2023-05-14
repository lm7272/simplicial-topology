use nalgebra::DMatrix;
use itertools::Itertools;
use std::collections::HashSet;
use num_integer::binomial;
use rayon::prelude::*;

use crate::utils::utils::{alternating_sum, filter_maximal_sets, remove_element};
use crate::utils::linear_algebra::{rank_smith_normal_matrix, row_nullity_smith_normal_matrix, gaussian_elimination};
use crate::simplicial_complex::simplex::{Simplex, Facet};

use super::simplex::{simplex_intersection, simplex_join};

#[derive(Debug)]
pub struct SimplicialComplex {
    facets: Vec<Facet>,
}

impl PartialEq for SimplicialComplex {
    fn eq(&self, other: &Self) -> bool {
        if self.facets.len() != other.facets.len() {
            return false;
        }
        for (_, facet) in self.facets.iter().enumerate() {
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

/// Memory efficient representation of simplicial complexes as a collection of facets of various dimensions.
impl SimplicialComplex {
    pub fn new(facets: Vec<Facet>) -> Self {
        Self::new_from_vec(facets.into_iter().map(|facet: Facet| facet.vertices).collect())
    }

    pub fn new_from_vec(v: Vec<Vec<usize>>) -> Self{
        let facets: Vec<Facet> = filter_maximal_sets(v).into_iter().map(|x| Simplex::new(x)).collect();
        Self { facets: facets.into_iter().map(|facet| facet.sort()).collect() }
    }

    /// Add simplex to complex. Panics if the boundary of this complex is not present.
    pub fn add_simplex(&mut self, simplex: Facet){
        let bdy = Self { facets: simplex.boundary() };
        if self.has_subcomplex(&bdy)
        {
            self.facets.retain(|facet| !bdy.facets.contains(facet));
            self.facets.push(simplex);
        }
        else {
            panic!("The boundary of {:?} is not contained in the simplicial complex.", simplex.vertices);
        }
    }

    /// Union of two complexes, returns a new complex
    pub fn union(&self, mut sc: Self) -> Self{
        let mut facets = self.facets.clone();
        facets.append(&mut sc.facets);
        return Self::new(facets)
    }

    /// Intersection of two complexes, returns a new complex
    pub fn intersection(self, sc: Self) -> Self{
        let mut facets: Vec<Facet> = Vec::new();
        for f in &self.facets{
            for g in &sc.facets{
                facets.push(simplex_intersection(f, g));
            }
        }
        Self::new(facets)
    }

    pub fn join(self, sigma: Facet, tau: Facet) -> Self{
        if simplex_intersection(&sigma, &tau).dimension() >= 0{
            panic!("Sigma and tau should be disjoint.")
        }
        let mut facets: Vec<Facet> = Vec::new();
        let mut sigma_count = 0;
        for facet in self.facets{
            if facet.has_subface(&sigma){
                let f = simplex_join(&sigma, &tau);
                facets.push(f);
                sigma_count += 1;
            }
            else{
                facets.push(facet);
            }
        }
        if sigma_count == 0{
            panic!("Sigma is not contained in the simplicial complex, join doesn't make sense.")
        }
        Self{facets}
    }

    pub fn k_external_faces(&self, dim: usize) -> Vec<Facet>{
        let k_minus_one_simplices = self.k_faces(dim-1);
        let k_simplices = self.k_faces(dim);
        let vertices: Vec<usize> = self.k_faces(0).iter().map(|v| v.vertices[0]).collect();
        let mut external_simplices = Self::new_from_vec(vec![vertices]).k_faces(dim);
        external_simplices.retain(|sigma| !k_simplices.contains(sigma));
        external_simplices.retain(|sigma| sigma.boundary().iter().all(|tau| k_minus_one_simplices.contains(tau)));
        external_simplices
    }

    fn external_faces(&self) -> Vec<Facet>{
        if &self.dimension() < &1{
            return Vec::new(); 
        }
        let dim: usize = self.dimension() as usize;
        (1..(dim+2)).into_par_iter().map(|dim| self.k_external_faces(dim)).flatten().collect()

    }

    /// Returns the combinatorial Alexander dual of the initial complex, where if \sigma is a simplex in X*
    /// iff the simplex defined by n - \sigma is not a simplex.
    /// 
    /// This has the reduced homology relation:
    /// H_i(X) \simeq H^{n-i-3}(X*),
    /// where n is the number of vertices of X.
    /// 
    /// As described here: 
    /// https://arxiv.org/pdf/0710.1172.pdf
    pub fn combinatorial_alexander_dual(&self) -> Self{
        let vertices: Vec<usize> = self.k_faces(0).iter().map(|v| v.vertices[0]).collect();
        let mut dual_facets: Vec<Facet> = Vec::new();
        
        for facet in &self.facets{
            let mut dual_bdy = facet.dual(&vertices).boundary();
            // dual_bdy.retain(|sigma| !self.has_subcomplex(&Self::new(vec![sigma.dual(&vertices)])));
            dual_facets.append(&mut dual_bdy);
        }
        for e_facet in self.external_faces(){
            let dual = e_facet.dual(&vertices);
            dual_facets.push(dual);
        }
        Self::new(dual_facets)
    }

    pub fn print(&self) {
        println!("Simplicial Complex has dimension {}. The facets are:", self.dimension());
        for facet in &self.facets {
            facet.print();
        }
    }


    pub fn dimension(&self) -> isize{
        if self.facets.is_empty(){
            return -1
        }
        self.facets.iter().map(|v| v.dimension()).max().unwrap()
    }
    
    pub fn is_pure(&self) -> bool {
        if self.dimension() < 0{
            return false
        }
        self.facets.iter().all(|facet| facet.dimension() == self.dimension())
    }

    pub fn k_skeleton(self, dim: usize) -> Self{
        // TODO: Can be improved by only doing self.k_faces for maximal faces of dimension > dim
        // and including all lower maximal faces
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

    pub fn compute_k_boundary_matrix(&self, dim:usize) -> DMatrix<i32> {
        //println!("Computing {}-dimensional boundary matrix", dim);
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

    pub fn has_subcomplex(&self, sc: &SimplicialComplex) -> bool{
        if sc.dimension() == -1 {
            return true
        }
        for f in &sc.facets{
            if !self.facets.iter().any(|x| x.has_subface(f)){
                return false
            }
        }
        true
    }

    pub fn contains_full_k_skeleton(&self, dim: usize) -> bool{
        if (self.dimension() < 0) | (dim > self.dimension() as usize){
            return false
        }
        let num_faces = self.k_faces(dim).len();
        num_faces == binomial(self.k_faces(0).len(), dim +1)
    }
    
    /// TODO: is there a more efficient method?
    pub fn is_connected(&self) -> bool{
        self.kth_betti_number(0) == 1
    }

    /// A simplicial complexes is minimally (path) connected if it is connected and the removal
    /// of any facet is disconnected (where we keep the same vertex set)
    pub fn is_minimal_connected(&self) -> bool {
        if !self.is_connected() {
            return false
        }
        let vertices = &self.k_faces(0);
        for (i,_) in self.facets.iter().enumerate(){
            let mut pruned_facets = remove_element(&mut self.facets.clone(), i);
            pruned_facets.append(&mut vertices.clone());
            let pruned_complex = Self {facets: pruned_facets };
            if pruned_complex.is_connected(){
                return false
            }
        }
        true
    }

    pub fn is_minimal_connected_par(&self) -> bool {
        if !self.is_connected() {
            return false
        }
        let vertices = &self.k_faces(0);
        !self.facets.par_iter().enumerate().any(|(i, _)| {
            let mut pruned_facets = remove_element(&mut self.facets.clone(), i);
            pruned_facets.append(&mut vertices.clone());
            let pruned_complex = Self {facets: pruned_facets };
            pruned_complex.is_connected()
        })
    }
    

    /// Computes the kth betti number of the complex by computing the k and (k+1) dimensional
    /// boundary matrices B_k and B_{k+1}. Then reduces these matrices by Gaussian elimination to the
    /// Smith normal form, SB_k, SB_{k+1}.
    /// b_k = row_null(SB_{k+1}) - rank(SB_k)
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