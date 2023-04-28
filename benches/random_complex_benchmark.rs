use criterion::{criterion_group, criterion_main, Criterion};
use itertools::Itertools;
use simplicial_topology::simplicial_complex::random_simplicial_complex::{generate_random_hypergraph, par_generate_random_hypergraph};

fn criterion_par_random_hypergraph_benchmark(c: &mut Criterion) {
    c.bench_function("parallel random hg", |b| b.iter(|| par_generate_random_hypergraph(30, vec![0.8,0.5,0.7,0.4])));
}

fn criterion_random_hypergraph_benchmark(c: &mut Criterion) {
    c.bench_function("random hg", |b| b.iter(|| generate_random_hypergraph(30, vec![0.8,0.5,0.7,0.4])));
}



criterion_group!(benches, criterion_random_hypergraph_benchmark, criterion_par_random_hypergraph_benchmark);
criterion_main!(benches);