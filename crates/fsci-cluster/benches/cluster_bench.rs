use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use fsci_cluster::{gaussian_mixture, kmeans, kmeans2};

/// Deterministic blobs: `n` points in `d` dims drawn around 4 cluster centres.
fn blobs(n: usize, d: usize) -> Vec<Vec<f64>> {
    (0..n)
        .map(|i| {
            let centre = (i % 4) as f64;
            (0..d)
                .map(|j| {
                    let t = (i * (j + 1)) as f64;
                    centre * 5.0 + (t * 0.013).sin() * 0.5 + ((i + j) % 7) as f64 * 0.05
                })
                .collect()
        })
        .collect()
}

/// kmeans (Lloyd) and kmeans2 (double-buffered Lloyd loop, frankenscipy-4ylee).
fn bench_kmeans(c: &mut Criterion) {
    let data = blobs(2000, 4);
    let init: Vec<Vec<f64>> = (0..4).map(|k| vec![k as f64 * 5.0; 4]).collect();
    let mut group = c.benchmark_group("kmeans");
    group.bench_function("k4/n2000", |b| b.iter(|| kmeans(&data, 4, 50, 42)));
    group.bench_function("kmeans2/k4/n2000", |b| b.iter(|| kmeans2(&data, &init, 50)));
    group.finish();
}

/// Gaussian-mixture EM. n=1000 is below the E-step work-gate (serial path);
/// n=5000/20000 exercise the parallel E-step (frankenscipy-yw7ts). Sizes mirror
/// docs/perf_oracle_gmm.py for the sklearn head-to-head.
fn bench_gmm(c: &mut Criterion) {
    let mut group = c.benchmark_group("gmm");
    for &(n, d, k) in &[(1000usize, 3usize, 3usize), (5000, 8, 5), (20000, 16, 8)] {
        let data = blobs(n, d);
        group.bench_function(BenchmarkId::new("diag", format!("n{n}_d{d}_k{k}")), |b| {
            b.iter(|| gaussian_mixture(&data, k, 50, 1e-4, 1e-6, 42))
        });
    }
    group.finish();
}

/// Hierarchical clustering: NN-chain linkage + cophenetic distances (the cophenet
/// member-list move-instead-of-clone win, frankenscipy-jphzn).
fn bench_hierarchical(c: &mut Criterion) {
    use fsci_cluster::{LinkageMethod, cophenet, linkage};
    let data = blobs(400, 4);
    let z = linkage(&data, LinkageMethod::Average).expect("linkage");
    let mut group = c.benchmark_group("hierarchical");
    group.bench_function("linkage_average/n400", |b| {
        b.iter(|| linkage(&data, LinkageMethod::Average))
    });
    group.bench_function("cophenet/n400", |b| b.iter(|| cophenet(&z)));
    group.finish();
}

/// Affinity propagation — the O(n²)-per-iteration responsibility/availability
/// message passing (responsibility update parallelized, frankenscipy-yw7ts).
fn bench_affinity_propagation(c: &mut Criterion) {
    use fsci_cluster::affinity_propagation;
    // n=300 is below the responsibility-update gate (n²<2¹⁸, serial); n=1000/2000
    // exercise the parallel update (frankenscipy-yw7ts). Mirrors docs/perf_oracle_ap.py.
    let mut group = c.benchmark_group("affinity_propagation");
    for &n in &[300usize, 1000, 2000] {
        let data = blobs(n, 4);
        let sim: Vec<Vec<f64>> = (0..n)
            .map(|i| {
                (0..n)
                    .map(|j| {
                        -data[i]
                            .iter()
                            .zip(&data[j])
                            .map(|(a, b)| (a - b).powi(2))
                            .sum::<f64>()
                    })
                    .collect()
            })
            .collect();
        group.bench_function(BenchmarkId::from_parameter(n), |b| {
            b.iter(|| affinity_propagation(&sim, -50.0, 0.9, 80, 15))
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_kmeans,
    bench_gmm,
    bench_hierarchical,
    bench_affinity_propagation
);
criterion_main!(benches);
