//! Byte-identity + timing harness for `vq` (vector quantization), whose
//! per-point nearest-centroid assignment is now parallel across points into
//! ordered slots (single pass) — byte-identical to the serial loop.
//!
//! Proof: labels + dists bits must be IDENTICAL across the stashed serial build.
//! Run it, `git stash` lib.rs, rebuild (serial), run again.
//! Run: `cargo run --release -p fsci-cluster --bin perf_vq`.

use std::hint::black_box;
use std::time::Instant;

use fsci_cluster::vq;

fn lcg(s: &mut u64) -> f64 {
    *s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
    (*s >> 11) as f64 / (1u64 << 53) as f64
}

fn points(n: usize, d: usize, seed: u64) -> Vec<Vec<f64>> {
    let mut s = seed;
    (0..n).map(|_| (0..d).map(|_| lcg(&mut s) * 10.0).collect()).collect()
}

fn digest(labels: &[usize], dists: &[f64]) -> u64 {
    let mut h = 1469598103934665603u64;
    for &l in labels {
        h = (h ^ l as u64).wrapping_mul(1099511628211);
    }
    for &dd in dists {
        h = (h ^ dd.to_bits()).wrapping_mul(1099511628211);
    }
    h
}

fn main() {
    println!("===GOLDEN_PAYLOAD_BEGIN===");
    for &(n, d, k) in &[(500usize, 4usize, 8usize), (2000, 8, 32), (5000, 16, 64)] {
        let data = points(n, d, 7);
        let cents = points(k, d, 99);
        let (labels, dists) = vq(&data, &cents).expect("vq");
        println!("n={n} d={d} k={k} digest={:016x}", digest(&labels, &dists));
    }
    println!("===GOLDEN_PAYLOAD_END===");

    for &(n, d, k) in &[(20000usize, 8usize, 32usize), (50000, 16, 64), (100000, 24, 128)] {
        let data = points(n, d, 7);
        let cents = points(k, d, 99);
        let reps = 5;
        let _ = vq(&data, &cents);
        let t0 = Instant::now();
        let mut acc = 0.0;
        for _ in 0..reps {
            let (_, dists) = vq(black_box(&data), &cents).unwrap();
            acc += dists[n / 2];
        }
        println!("n={n} d={d} k={k}  {:>10.3?}/call (acc={acc:.6})", t0.elapsed() / reps);
    }
}
