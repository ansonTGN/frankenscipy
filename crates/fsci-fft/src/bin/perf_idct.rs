//! Correctness + timing harness for `idct` (DCT-III), which now uses an
//! N/2-point real inverse FFT (inverse Makhoul reorder) instead of a 2N-point
//! complex inverse FFT of the Hermitian-extended spectrum.
//!
//! Correctness: idct(dct(x)) must reproduce x (roundtrip) to ~machine eps, well
//! under the 1e-9 parity tolerance. The same `idct` public API is timed, so this
//! harness is build-agnostic: run it, `git stash` the transforms.rs edit,
//! rebuild (old 2N path), and run again for the speedup.
//! Run: `cargo run --release -p fsci-fft --bin perf_idct`.

use std::hint::black_box;
use std::time::Instant;

use fsci_fft::{FftOptions, dct, idct};

fn lcg(s: &mut u64) -> f64 {
    *s = s
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);
    (*s >> 11) as f64 / (1u64 << 53) as f64 * 2.0 - 1.0
}

fn signal(n: usize, seed: u64) -> Vec<f64> {
    let mut s = seed;
    (0..n).map(|_| lcg(&mut s)).collect()
}

fn main() {
    let opts = FftOptions::default();

    println!("===GOLDEN_PAYLOAD_BEGIN===");
    let mut worst = 0.0f64;
    for &n in &[2usize, 4, 6, 8, 12, 16, 24, 32, 60, 64, 100, 128, 360, 1000] {
        let x = signal(n, n as u64 * 6271 + 5);
        let coeffs = dct(&x, &opts).expect("dct");
        let back = idct(&coeffs, &opts).expect("idct");
        let err = x
            .iter()
            .zip(&back)
            .map(|(a, b)| (a - b).abs())
            .fold(0.0f64, f64::max);
        worst = worst.max(err);
        let sum: f64 = back.iter().sum();
        println!("n={n:>4} roundtrip_err={err:.3e} sum={sum:+.9e}");
    }
    println!("===GOLDEN_PAYLOAD_END===");
    println!("worst idct(dct(x)) roundtrip err = {worst:.3e} (parity tol 1e-9)");
    assert!(worst < 1e-9, "idct roundtrip exceeds parity tolerance");

    for &n in &[512usize, 1000, 2048, 4096, 10000, 16384] {
        let coeffs = signal(n, 99);
        let reps = 500;
        let _ = idct(&coeffs, &opts).unwrap();
        let t0 = Instant::now();
        let mut acc = 0.0;
        for _ in 0..reps {
            let r = idct(black_box(&coeffs), &opts).unwrap();
            acc += r[r.len() / 2];
        }
        let dt = t0.elapsed() / reps;
        println!("n={n:>6} {dt:>10.3?}/call (acc={acc:.3})");
    }
}
