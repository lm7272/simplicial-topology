use criterion::{criterion_group, criterion_main, Criterion};
use itertools::Itertools;
use simplicial_topology::{simplicial_complex::{simplex::{Simplex, Facet}, simplicial_complex::SimplicialComplex, hypergraph::Hypergraph}, utils::utils::get_subvectors};

// TODO: benchmark is minimal connected cover

fn criterion_par_downward_closure_benchmark(c: &mut Criterion) {
    let vertices = (0..10).collect_vec();
    let mut hyperedges: Vec<Vec<usize>> = Vec::new();
    for i in 2..7 {
        hyperedges.extend(get_subvectors(&vertices, i));
    }
    let hg = Hypergraph {vertices,hyperedges};
    c.bench_function("hg parallel downward closure", |b| b.iter(|| hg.clone().par_downward_closure()));
}

fn criterion_downward_closure_benchmark(c: &mut Criterion) {
    let vertices = (0..10).collect_vec();
    let mut hyperedges: Vec<Vec<usize>> = Vec::new();
    for i in 2..7 {
        hyperedges.extend(get_subvectors(&vertices, i));
    }
    let hg = Hypergraph {vertices,hyperedges};
    c.bench_function("hg downward closure", |b| b.iter(|| hg.clone().downward_closure()));
}

fn criterion_upward_closure_benchmark(c: &mut Criterion) {
    let vertices = (0..10).collect_vec();
    let mut hyperedges: Vec<Vec<usize>> = Vec::new();
    for i in 2..7 {
        hyperedges.extend(get_subvectors(&vertices, i));
    }
    let hg = Hypergraph {vertices,hyperedges};
    c.bench_function("hg upward closure", |b| b.iter(|| hg.clone().upward_closure()));
}

fn criterion_betti_benchmark(c: &mut Criterion) {
    let simplex: Facet = Simplex::new((0..10).collect_vec());
    let sc = SimplicialComplex::new(simplex.boundary());
    c.bench_function("betti vector", |b| b.iter(|| sc.betti_numbers()));
}

criterion_group!(benches, criterion_betti_benchmark, criterion_downward_closure_benchmark, criterion_par_downward_closure_benchmark, criterion_upward_closure_benchmark);
criterion_main!(benches);