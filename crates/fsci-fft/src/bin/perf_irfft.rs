//! Correctness + timing harness for `irfft`, whose inner `real_ifft_unscaled`
//! now inverts the real-FFT pack with a single N/2-point complex IFFT (mirror of
//! the forward `real_fft_specialized`) instead of a full N-point complex IFFT.
//!
//! Correctness: irfft(rfft(x)) must reproduce x (roundtrip) to ~machine eps,
//! well under the 1e-9 parity tolerance. The same `irfft` public API is timed,
//! so this harness is build-agnostic: run it, `git stash` the transforms.rs
//! edit, rebuild (old full-N IFFT), and run again for the speedup.
//! Run: `cargo run --release -p fsci-fft --bin perf_irfft`.

use std::hint::black_box;
use std::time::Instant;

use fsci_fft::{FftOptions, irfft, rfft};

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
    for &n in &[4usize, 6, 8, 12, 16, 24, 32, 60, 64, 100, 128, 360, 1000] {
        let x = signal(n, n as u64 * 7919 + 3);
        let spectrum = rfft(&x, &opts).expect("rfft");
        let back = irfft(&spectrum, Some(n), &opts).expect("irfft");
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
    println!("worst irfft(rfft(x)) roundtrip err = {worst:.3e} (parity tol 1e-9)");
    assert!(worst < 1e-9, "irfft roundtrip exceeds parity tolerance");

    for &n in &[512usize, 1000, 2048, 4096, 10000, 16384] {
        let x = signal(n, 99);
        let spectrum = rfft(&x, &opts).expect("rfft");
        let reps = 500;
        let _ = irfft(&spectrum, Some(n), &opts).unwrap();
        let t0 = Instant::now();
        let mut acc = 0.0;
        for _ in 0..reps {
            let r = irfft(black_box(&spectrum), Some(n), &opts).unwrap();
            acc += r[r.len() / 2];
        }
        let dt = t0.elapsed() / reps;
        println!("n={n:>6} {dt:>10.3?}/call (acc={acc:.3})");
    }
}
