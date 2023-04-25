use criterion::{criterion_group, criterion_main, Criterion};
use itertools::Itertools;
use simplicial_topology::simplicial_complex::{simplex::{Simplex, Facet}, simplicial_complex::SimplicialComplex};

fn criterion_betti_benchmark(c: &mut Criterion) {
    let simplex: Facet = Simplex::new((0..10).collect_vec());
    let sc = SimplicialComplex::new(simplex.boundary());
    c.bench_function("betti vector", |b| b.iter(|| sc.betti_numbers()));
}

criterion_group!(benches, criterion_betti_benchmark);
criterion_main!(benches);