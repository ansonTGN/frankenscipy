//! Correctness + timing harness for the mixed-radix Cooley-Tukey path that now
//! serves composite non-power-of-2 FFT lengths (previously all of these routed
//! through Bluestein on a size `next_pow2(2n-1)` convolution).
//!
//! Correctness is checked against a naive O(n²) DFT ground truth (max abs error
//! must be ~machine eps · n, well under the 1e-9 scipy-parity tolerance). The
//! same `fft` public API is timed, so this harness is build-agnostic: run it on
//! the new build, `git stash` the mixed-radix edit, rebuild (Bluestein), and run
//! again to read the speedup.
//! Run: `cargo run --release -p fsci-fft --bin perf_mixed_radix`.

use std::hint::black_box;
use std::time::Instant;

use fsci_fft::{Complex64, FftOptions, fft};

fn naive_dft(input: &[Complex64]) -> Vec<Complex64> {
    let n = input.len();
    let mut out = vec![(0.0f64, 0.0f64); n];
    for (k, slot) in out.iter_mut().enumerate() {
        let mut re = 0.0;
        let mut im = 0.0;
        for (j, &(xr, xi)) in input.iter().enumerate() {
            let angle = -2.0 * std::f64::consts::PI * (k * j % n) as f64 / n as f64;
            let (s, c) = angle.sin_cos();
            re += xr * c - xi * s;
            im += xr * s + xi * c;
        }
        *slot = (re, im);
    }
    out
}

fn lcg(s: &mut u64) -> f64 {
    *s = s
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);
    (*s >> 11) as f64 / (1u64 << 53) as f64 * 2.0 - 1.0
}

fn signal(n: usize, seed: u64) -> Vec<Complex64> {
    let mut s = seed;
    (0..n).map(|_| (lcg(&mut s), lcg(&mut s))).collect()
}

fn main() {
    let opts = FftOptions::default();

    // ---- Correctness vs naive DFT (small composite sizes) ----
    println!("===GOLDEN_PAYLOAD_BEGIN===");
    let mut worst = 0.0f64;
    for &n in &[
        6usize, 9, 12, 15, 20, 24, 30, 36, 45, 60, 100, 105, 120, 210, 360,
    ] {
        let x = signal(n, n as u64 * 2718 + 1);
        let got = fft(&x, &opts).expect("fft");
        let want = naive_dft(&x);
        let err = got
            .iter()
            .zip(&want)
            .map(|(a, b)| ((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2)).sqrt())
            .fold(0.0f64, f64::max);
        worst = worst.max(err);
        // Round to a stable digest field so the golden payload is reproducible.
        let scaled = got.iter().map(|c| c.0 + c.1).sum::<f64>();
        println!("n={n:>4} maxerr={err:.3e} sum={scaled:+.9e}");
    }
    println!("===GOLDEN_PAYLOAD_END===");
    println!("worst maxerr vs naive DFT = {worst:.3e} (parity tol 1e-9)");
    assert!(worst < 1e-9, "mixed-radix FFT exceeds parity tolerance");

    // ---- Timing: composite (smooth) non-power-of-2 lengths ----
    for &n in &[720usize, 1000, 1080, 1500, 1920, 3000, 5000, 10000] {
        let x = signal(n, 42);
        let reps = 200;
        // warm plan/twiddle caches
        let _ = fft(&x, &opts).unwrap();
        let t0 = Instant::now();
        let mut acc = 0.0;
        for _ in 0..reps {
            let r = fft(black_box(&x), &opts).unwrap();
            acc += r[r.len() / 2].0;
        }
        let dt = t0.elapsed() / reps;
        let factored = factor_string(n);
        println!("n={n:>6} ({factored:<14}) {dt:>10.3?}/call (acc={acc:.3})");
    }
}

fn factor_string(mut n: usize) -> String {
    let mut parts = Vec::new();
    let mut f = 2;
    while f * f <= n {
        while n.is_multiple_of(f) {
            parts.push(f.to_string());
            n /= f;
        }
        f += 1;
    }
    if n > 1 {
        parts.push(n.to_string());
    }
    parts.join("*")
}
