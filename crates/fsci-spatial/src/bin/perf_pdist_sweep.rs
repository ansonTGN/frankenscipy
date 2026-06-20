use fsci_spatial::{DistanceMetric, pdist};
use std::hint::black_box;
use std::time::Instant;

fn data(n: usize, dim: usize) -> Vec<Vec<f64>> {
    (0..n)
        .map(|i| {
            (0..dim)
                .map(|k| ((i as f64) * (0.1 + 0.01 * k as f64)).sin())
                .collect()
        })
        .collect()
}
fn time_ms(iters: usize, mut f: impl FnMut() -> Vec<f64>) -> f64 {
    black_box(f());
    let s = Instant::now();
    for _ in 0..iters {
        black_box(f());
    }
    s.elapsed().as_secs_f64() * 1e3 / iters as f64
}
fn main() {
    use DistanceMetric::*;
    let metrics = [
        ("euclidean", Euclidean),
        ("cityblock", Cityblock),
        ("sqeuclidean", SqEuclidean),
        ("chebyshev", Chebyshev),
    ];
    for (n, dim) in [(512usize, 4usize), (512, 16), (512, 64)] {
        let x = data(n, dim);
        for (name, m) in metrics {
            let ms = time_ms(200, || pdist(&x, m).unwrap());
            println!("rust pdist/{name}/n{n}/d{dim}: {ms:.3} ms");
        }
    }
    // regression checks: large-n euclidean/cosine d4 (was 64-thread parallel) + big chebyshev
    for (n, dim, name, m) in [
        (4096usize, 4usize, "euclidean", Euclidean),
        (4096, 4, "cosine", Cosine),
        (2048, 64, "chebyshev", Chebyshev),
        (2048, 64, "cityblock", Cityblock),
    ] {
        let x = data(n, dim);
        let ms = time_ms(20, || pdist(&x, m).unwrap());
        println!("rust[big] pdist/{name}/n{n}/d{dim}: {ms:.3} ms");
    }
}
