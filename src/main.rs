use simplicial_topology::simplicial_complex::random_simplicial_complex::{Model, generate_many_random_betti_numbers};
use simplicial_topology::{sc, simplex};
use simplicial_topology::simplicial_complex::simplex::{Simplex, Facet};
use simplicial_topology::simplicial_complex::simplicial_complex::SimplicialComplex;
use simplicial_topology::graphics::plot::betti_number_histogram;

fn main() {
    println!("generating random hypergraph");
    // let hg = generate_random_hypergraph(30, vec![1.0,0.7,0.8,0.1]);
    // let sc = generate_random_simplicial_complex(Model::Pure { num_vertices: 30, dimension: 3, prob: 0.1, include_all_vertices: true });
    // println!("Pure random complex has Betti vector is: {:?}", sc.betti_numbers());
    
    // let sc = hg.clone().par_downward_closure();
    // println!("Downward closure Betti vector is: {:?}", sc.betti_numbers());
    // println!("This simplicial complex has Euler characteristic: {}", sc.euler_characteristic());

    // let sc = hg.clone().upward_closure();
    // println!("Upward closure Betti vector is: {:?}", sc.betti_numbers());
    // println!("This simplicial complex has Euler characteristic: {}", sc.euler_characteristic());
    let mut sc: SimplicialComplex = sc![vec![1,2], vec![2,3], vec![1,3], vec![1,4], vec![4,5], vec![1,5]];
    let sigma: Facet = simplex![1,2,3];
    let tau: Facet = simplex![1,4,5];
    let sc2: SimplicialComplex = sigma.boundary_as_complex().union(tau.boundary_as_complex());
    //let sc2 = SimplicialComplex::new(bdy1);
    assert_eq!(sc, sc2);
    println!("External faces: {:?}", sc.k_external_faces(2));
    sc.add_simplex(simplex![1,2,3]);
    sc.add_simplex(simplex![6]);
    sc.print();
    println!("Betti vector is: {:?}", sc.betti_numbers());
    println!("Simplicial complex is minimal cover: {:?}", sc.is_minimal_connected());
    println!("This simplicial complex has Euler characteristic: {}", sc.euler_characteristic());
    let sc2 = sc.combinatorial_alexander_dual();
    sc2.print();    
    println!("Betti vector is: {:?}", sc2.betti_numbers());
    let n: usize = 20;
    let prob_vec: Vec<f64> = vec![1.0, 1.0/(n as f64).powf(0.5), 1.0];
    let model = Model::Lower {num_vertices: n, prob_vec: prob_vec };
    let betti_numbers: Vec<Vec<i32>> = generate_many_random_betti_numbers(1000, model);
    println!("{:?}", betti_numbers);
    let plot = betti_number_histogram(&betti_numbers);
    plot.write_html("out.html");
}