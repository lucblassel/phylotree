use criterion::BenchmarkId;
use criterion::Criterion;
use criterion::{criterion_group, criterion_main};

use phylotree::{generate_tree, tree::Tree};

fn distance_naive(tree: &mut Tree) {
    let _matrix = tree.distance_matrix().unwrap();
}

fn distance_recurs(tree: &Tree) {
    let _matrix = tree.distance_matrix_recursive().unwrap();
}

fn from_elem(c: &mut Criterion) {
    let mut tree: Tree = generate_tree(100, true, phylotree::distr::Distr::Uniform).unwrap();

    c.bench_with_input(
        BenchmarkId::new("input_uncached", tree.size()),
        &tree,
        |b, s| {
            let mut tree = s.clone();
            b.iter(|| distance_naive(&mut tree));
        },
    );

    c.bench_with_input(
        BenchmarkId::new("input_recursive", tree.size()),
        &tree,
        |b, s| {
            b.iter(|| distance_recurs(s));
        },
    );
}

criterion_group!(benches, from_elem);
criterion_main!(benches);
