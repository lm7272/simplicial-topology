use itertools::Itertools;
use simplicial_topology::simplicial_complex::simplicial_complex::SimplicialComplex;
use simplicial_topology::simplicial_complex::hypergraph::{Hypergraph, upward_closure, downward_closure};

fn main() {
    let hg = Hypergraph {
        vertices: vec![1, 2, 3, 4, 6],
        hyperedges: vec![
            vec![1, 2],
            vec![2, 3],
            vec![3, 4],
            vec![4, 1],
            vec![2, 4],
            vec![1,5],
            vec![1,2,3]
        ],
    };
    print!("{:?}\n", hg.hyperedges);
    downward_closure(hg.clone()).print();
    upward_closure(hg).print();
    // let sc:SimplicialComplex = upward_closure(hg);
    // sc.print();
    // //let sc:SimplicialComplex = SimplicialComplex::new_from_vec(vec![(0..15).collect_vec()]).k_skeleton(5);
    // // sc.print(); 
    // println!("Betti vector is: {:?}", sc.betti_numbers());
    // println!("This simplicial complex has Euler characteristic: {}", sc.euler_characteristic());
}