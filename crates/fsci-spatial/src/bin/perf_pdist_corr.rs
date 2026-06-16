//! Same-process A/B + byte-identity for pdist Cosine/Correlation: the new precompute-once
//! path (library) vs an inline old-style per-pair reference (recomputes norms/means each
//! pair, parallel like the old pdist). Byte-identical (digest match). Run:
//! `cargo run --release -p fsci-spatial --bin perf_pdist_corr`.

use std::hint::black_box;
use std::time::Instant;

use fsci_spatial::{DistanceMetric, correlation, cosine, pdist};

fn time<F: FnMut()>(reps: usize, mut f: F) -> f64 {
    let t = Instant::now();
    for _ in 0..reps {
        f();
    }
    t.elapsed().as_secs_f64() * 1e3 / reps as f64
}

fn digest(v: &[f64]) -> u64 {
    v.iter().fold(1469598103934665603u64, |h, &x| {
        (h ^ x.to_bits()).wrapping_mul(1099511628211)
    })
}

// Old-style pdist: per-pair metric call (recomputes per-vector quantities), parallel rows.
fn old_pdist(x: &[Vec<f64>], f: fn(&[f64], &[f64]) -> f64) -> Vec<f64> {
    let n = x.len();
    let total = n * (n - 1) / 2;
    let mut out = vec![0.0f64; total];
    let off = |r: usize| -> usize { r * (n - 1) - r * (r.saturating_sub(1)) / 2 };
    let nth = std::thread::available_parallelism()
        .map(|c| c.get())
        .unwrap_or(1)
        .min(n.max(1));
    let chunk_rows = n.div_ceil(nth);
    std::thread::scope(|s| {
        let mut rest: &mut [f64] = &mut out;
        let mut prev = 0usize;
        for w in 0..nth {
            let r0 = (w * chunk_rows).min(n);
            let r1 = ((w + 1) * chunk_rows).min(n);
            if r0 >= r1 {
                break;
            }
            let take = off(r1) - prev;
            prev = off(r1);
            let (seg, tail) = rest.split_at_mut(take);
            rest = tail;
            s.spawn(move || {
                let mut local = 0usize;
                for i in r0..r1 {
                    for j in (i + 1)..n {
                        seg[local] = f(&x[i], &x[j]);
                        local += 1;
                    }
                }
            });
        }
    });
    out
}

fn main() {
    let mut s = 0xC0FFEEu64;
    let mut next = || {
        s = s
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        (s >> 11) as f64 / (1u64 << 53) as f64
    };
    for &(n, d) in &[(800usize, 64usize), (1500, 128), (2500, 256)] {
        let x: Vec<Vec<f64>> = (0..n).map(|_| (0..d).map(|_| next()).collect()).collect();
        for (name, metric, f) in [
            (
                "cosine",
                DistanceMetric::Cosine,
                cosine as fn(&[f64], &[f64]) -> f64,
            ),
            (
                "correl",
                DistanceMetric::Correlation,
                correlation as fn(&[f64], &[f64]) -> f64,
            ),
        ] {
            let new = pdist(&x, metric).unwrap();
            let old = old_pdist(&x, f);
            let bit = digest(&new) == digest(&old);
            let reps = 6usize;
            let t_new = time(reps, || {
                black_box(pdist(black_box(&x), metric).unwrap());
            });
            let t_old = time(reps, || {
                black_box(old_pdist(black_box(&x), f));
            });
            println!(
                "pdist {name} n={n:>5} d={d:>4}: old={t_old:>8.4}ms  new={t_new:>8.4}ms  speedup={:>6.2}x  bit_identical={bit}",
                t_old / t_new
            );
        }
    }
}
