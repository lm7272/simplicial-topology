use plotly::common::Title;
use plotly::layout::RelayoutLayout;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use simplicial_topology::simplicial_complex::random_simplicial_complex::{Model, generate_random_simplicial_complex};
use simplicial_topology::{sc, simplex};
use simplicial_topology::simplicial_complex::simplex::{Simplex, Facet};
use simplicial_topology::simplicial_complex::simplicial_complex::SimplicialComplex;
use plotly::{
    Histogram,
    common::Visible,
    layout::{
        update_menu::{ButtonBuilder, UpdateMenu},
    },
    Layout, Plot
};

fn betti_number_histogram(betti_numbers_vec: &Vec<Vec<i32>>) {
    let n = betti_numbers_vec[0].len();
    let mut plot = Plot::new();
    for k in 0..n{
        let mut tmp_betti_numbers: Vec<i32> = Vec::new();
        for betti_numbers in betti_numbers_vec.iter(){
            tmp_betti_numbers.push(betti_numbers[k]);
        }
        let visible = match k {
            0 => Visible::True,
            _ => Visible::False
        };
        let k_str: &str = &k.to_string();
        let trace = Histogram::new(tmp_betti_numbers).name(k_str).visible(visible);
        plot.add_trace(trace);
    }

    let mut buttons = Vec::new();
    for k in 0..n{
        let k_str: &str = &k.to_string();
        let visibility = (0..n).into_iter().map(|i| if i == k { Visible::True } else { Visible::False }).collect::<Vec<_>>();
        let title = "Betti number: ".to_owned() + k_str;
        let title_str = title.as_str();
        let layout = RelayoutLayout::ModifyTitle { title: Some(Title::new(title_str)) };
        let btn = ButtonBuilder::new()
            .label(k_str)
            .push_restyle(Histogram::<i32>::modify_visible(visibility))
            .push_relayout(layout)
            .build();
        buttons.push(btn);
    }
    let layout = Layout::new().title(Title::new("Betti number: 0")).update_menus(vec![UpdateMenu::new().y(0.8).buttons(buttons)]);
    plot.set_layout(layout);

    plot.write_html("out.html");
}

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
    sc.add_simplex(simplex![1,4,5]);
    sc.add_simplex(simplex![1,2,3]);
    println!("Betti vector is: {:?}", sc.betti_numbers());
    println!("Simplicial complex is minimal cover: {:?}", sc.is_minimal_connected());
    println!("This simplicial complex has Euler characteristic: {}", sc.euler_characteristic());
    let n: usize = 20;
    let prob_vec: Vec<f64> = vec![1.0, 1.0/(n as f64).powf(0.5), 1.0];
    let model = Model::Lower {num_vertices: n, prob_vec: prob_vec };
    let complexes: Vec<SimplicialComplex> = (0..1000).collect::<Vec<usize>>().into_par_iter().map(|_| generate_random_simplicial_complex(&model)).collect();
    // println!("{:?}", complexes);
    let betti_numbers: Vec<Vec<i32>> = complexes.into_par_iter().map(|sc| sc.betti_numbers()).collect();
    println!("{:?}", betti_numbers);
    betti_number_histogram(&betti_numbers);   
}