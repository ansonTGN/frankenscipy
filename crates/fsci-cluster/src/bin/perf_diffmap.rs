// A/B for diffusion_map (data-based, O(n·m) — never forms the n×n affinity) vs the full path:
// build the dense n×n RBF affinity, take spectral_embedding (randomized eigh), then scale by
// μ^t. Both produce diffusion coordinates that separate the blobs; the speedup is the
// wall-clock ratio (the O(n²)→O(n·m) drop from skipping the dense affinity).
use fsci_cluster::{diffusion_map, kmeans, spectral_embedding};
use std::hint::black_box;
use std::time::Instant;

fn purity(labels: &[usize], truth: &[usize], k: usize) -> f64 {
    let n = labels.len();
    let mut correct = 0usize;
    for pred in 0..k {
        let mut counts = vec![0usize; k];
        for i in 0..n {
            if labels[i] == pred {
                counts[truth[i]] += 1;
            }
        }
        correct += counts.iter().copied().max().unwrap_or(0);
    }
    correct as f64 / n as f64
}

fn full_diffusion(pts: &[Vec<f64>], k: usize, gamma: f64, t: f64) -> Vec<Vec<f64>> {
    let n = pts.len();
    let aff: Vec<Vec<f64>> = (0..n)
        .map(|i| {
            (0..n)
                .map(|j| {
                    let d2: f64 = pts[i]
                        .iter()
                        .zip(&pts[j])
                        .map(|(&a, &b)| (a - b) * (a - b))
                        .sum();
                    (-gamma * d2).exp()
                })
                .collect()
        })
        .collect();
    let se = spectral_embedding(&aff, k + 1, 7).expect("se");
    // drop trivial col 0, scale by μ^t
    (0..n)
        .map(|i| {
            (1..=k)
                .map(|j| se.embedding[i][j] * se.eigenvalues[j].max(0.0).powf(t))
                .collect()
        })
        .collect()
}

fn main() {
    let k = 4usize;
    let per = 700usize;
    let n = k * per;
    let m = 48usize;
    let gamma = 0.5f64;
    let t = 1.0f64;
    let mut st: u64 = 0x243f_6a88_85a3_08d3;
    let mut rng = || {
        st = st
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        ((st >> 11) as f64) / (1u64 << 53) as f64 - 0.5
    };
    let mut pts = Vec::new();
    let mut truth = Vec::new();
    for c in 0..k {
        for _ in 0..per {
            pts.push(vec![15.0 * c as f64 + rng(), rng()]);
            truth.push(c);
        }
    }

    let dm = diffusion_map(&pts, k, m, gamma, t, 7).expect("diffusion_map");
    let labels = kmeans(&dm.embedding, k, 100, 7).expect("km").labels;
    println!(
        "diffusion_map purity={:.3}  (n={n} m={m})",
        purity(&labels, &truth, k)
    );

    let trials = 3;
    let mut td = Vec::new();
    let mut tf = Vec::new();
    for _ in 0..trials {
        let s = Instant::now();
        black_box(diffusion_map(&pts, k, m, gamma, t, 7).unwrap());
        td.push(s.elapsed().as_secs_f64());
        let s = Instant::now();
        black_box(full_diffusion(&pts, k, gamma, t));
        tf.push(s.elapsed().as_secs_f64());
    }
    td.sort_by(|a, b| a.partial_cmp(b).unwrap());
    tf.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let d_ms = td[trials / 2] * 1e3;
    let f_ms = tf[trials / 2] * 1e3;
    println!(
        "full affinity diffusion {f_ms:.2} ms | diffusion_map {d_ms:.2} ms | speedup {:.2}x  (n={n} m={m} k={k})",
        f_ms / d_ms
    );
}
