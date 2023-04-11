use nalgebra::{DMatrix};

trait Simplex {
    type Boundary: Sized;
    fn new(vertices: Vec<usize>) -> Self;
    fn facet(&mut self, vertices: Vec<usize>);
    fn dimension(&self) -> usize;
    fn boundary(&self) -> Self::Boundary;
    fn print(&self);
}

#[derive(Debug)]
struct Facet{
    vertices: Vec<usize>
}
struct SimplicialComplex {
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
        // let n: usize = self.dimension();
        println!("{:?}", self.vertices);
        // This has boundary simplices:", self.vertices, n);
        // let _boundary: Vec<Self> = self.boundary();
        // for face in _boundary {
        //     println!("{:?}", face.vertices)
        // }
    }
}

impl SimplicialComplex {
    fn new(facets: Vec<Facet>) -> Self {
        Self { facets }
    }

    fn print(&self) {
        println!("Simplicial Complex:");
        for facet in &self.facets {
            facet.print();
        }
    }
}

fn compute_boundary_matrix(k_simplices: &Vec<Facet>, k_plus_one_simplices: &Vec<Facet>) -> DMatrix<i32> {
    let m = k_simplices.len();
    let n = k_plus_one_simplices.len();
    let mut matrix = DMatrix::from_element(m, n, 0);
    // Populate the matrix based on whether k-simplices are in the boundary of a k+1 simplex
    for (i, facet) in k_plus_one_simplices.iter().enumerate() {
        let bdy = facet.boundary();
        for simplex in bdy {
            let index = k_simplices.iter().position(|x| simplex == x);
            if let Some(j) = index {
                matrix[(j, i)] = 1
            }
        }
    }

    matrix
}

fn main() {
    let facets: Vec<Facet> = vec![Simplex::new(vec![0,1]), Simplex::new(vec![1,2]), Simplex::new(vec![0,2])];
    let bdy: Vec<Facet> = vec![Simplex::new(vec![0]), Simplex::new(vec![1]), Simplex::new(vec![2])];
    let m = compute_boundary_matrix(&bdy, &facets);
    println!("{}", m);
}

// fn main() {
//     let simplex: Facet = Simplex::new(vec![0,1,2,3,4]);
//     simplex.print()
// }