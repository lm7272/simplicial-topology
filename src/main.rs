use simplicial_topology::simplicial_complex::simplex::Simplex;
use simplicial_topology::simplicial_complex::simplicial_complex::SimplicialComplex;
use simplicial_topology::simplicial_complex::hypergraph::{Hypergraph};
use simplicial_topology::simplicial_complex::random_simplicial_complex::{generate_random_hypergraph, generate_random_simplicial_complex, Model};

fn main() {
    let hg = Hypergraph {
        vertices: vec![1, 2, 3, 4, 5, 6],
        hyperedges: vec![
            vec![1, 2],
            vec![2, 3],
            vec![1, 3],
            vec![1, 4],
            vec![2, 4],
            vec![1, 5],
            vec![1, 3, 4],
            vec![1, 2, 4]
        ],
    };
    std::env::set_var("RUST_BACKTRACE", "1");
    println!("generating random hypergraph");
    let hg = generate_random_hypergraph(50, vec![1.0,1.0,1.0,0.1]);
    let sc = generate_random_simplicial_complex(Model::LinialMeshulam { num_vertices: 20, dimension: 3, prob: 0.1 });
    sc.print();
    println!("Betti vector is: {:?}", sc.betti_numbers());
    //print!("Vertices: {:?}\nHyperedges: {:?}\n", hg.vertices, hg.hyperedges);
    let simplicial_complex = SimplicialComplex::new(vec![Simplex::new((0..5).collect()),Simplex::new((0..7).collect())]);
    simplicial_complex.print();
    //simplicial_complex.contains_full_k_skeleton(4);
    //let sc = SimplicialComplex::new(simplex.boundary());
    let sc = hg.clone().downward_closure();
    sc.print();
    println!("Betti vector is: {:?}", sc.betti_numbers());
    println!("This simplicial complex has Euler characteristic: {}", sc.euler_characteristic());
    // let sc2 = hg.upward_closure();
    // sc2.print();
    // println!("Betti vector is: {:?}", sc2.betti_numbers());
    // println!("This simplicial complex has Euler characteristic: {}", sc2.euler_characteristic());
    // //sc.clone().star(&Simplex::new(vec![1])).print();
    //sc.link(&Simplex::new(vec![1])).print();
    // let sc:SimplicialComplex = upward_closure(hg);
    // sc.print();
    // //let sc:SimplicialComplex = SimplicialComplex::new_from_vec(vec![(0..15).collect_vec()]).k_skeleton(5);
    // // sc.print(); 
}