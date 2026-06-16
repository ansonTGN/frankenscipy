//! Byte-identity + timing harness for `elbow_inertias`, whose per-k kmeans runs
//! are now computed in parallel into ordered slots (each k uses a deterministic
//! seed+k, no cross-k reduction) — byte-identical to the serial map.
//!
//! Proof: the inertia vector bits must be IDENTICAL across the stashed serial
//! build. Run it, `git stash` lib.rs, rebuild (serial), run again.
//! Run: `cargo run --release -p fsci-cluster --bin perf_elbow`.

use std::hint::black_box;
use std::time::Instant;

use fsci_cluster::elbow_inertias;

fn lcg(s: &mut u64) -> f64 {
    *s = s
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);
    (*s >> 11) as f64 / (1u64 << 53) as f64
}

fn dataset(n: usize, d: usize, seed: u64) -> Vec<Vec<f64>> {
    let mut s = seed;
    (0..n)
        .map(|_| {
            let c = (lcg(&mut s) * 4.0).floor();
            (0..d).map(|_| c * 5.0 + lcg(&mut s)).collect()
        })
        .collect()
}

fn main() {
    println!("===GOLDEN_PAYLOAD_BEGIN===");
    for &(n, d, max_k) in &[(150usize, 3usize, 8usize), (400, 5, 12)] {
        let data = dataset(n, d, 7);
        for seed in [42u64, 12345] {
            let v = elbow_inertias(&data, max_k, seed);
            let bits: Vec<String> = v.iter().map(|x| format!("{:016x}", x.to_bits())).collect();
            println!(
                "n={n} d={d} max_k={max_k} seed={seed} inertias={}",
                bits.join(",")
            );
        }
    }
    println!("===GOLDEN_PAYLOAD_END===");

    for &(n, d, max_k) in &[(400usize, 5usize, 12usize), (800, 8, 20)] {
        let data = dataset(n, d, 7);
        let reps = 3;
        let _ = elbow_inertias(&data, max_k, 1);
        let t0 = Instant::now();
        let mut acc = 0.0;
        for _ in 0..reps {
            let v = elbow_inertias(black_box(&data), max_k, 1);
            acc += v.iter().sum::<f64>();
        }
        println!(
            "n={n} d={d} max_k={max_k}  {:>10.3?}/call (acc={acc:.6})",
            t0.elapsed() / reps
        );
    }
}
