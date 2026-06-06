//! Correctness + timing harness for DCT-IV (`dct_iv`) and DST-IV (`dst_iv`),
//! which now use one 2N-point complex FFT (pre/post half-sample rotation)
//! instead of the old 8N-point complex FFT.
//!
//! Correctness is checked against naive O(n²) DCT-IV / DST-IV ground truth (max
//! abs error must be ~machine eps·n, well under the 1e-9 parity tolerance). The
//! same public APIs are timed, so this harness is build-agnostic: run it, `git
//! stash` the transforms.rs edit, rebuild (old 8N path), and run again.
//! Run: `cargo run --release -p fsci-fft --bin perf_dct4`.

use std::hint::black_box;
use std::time::Instant;

use fsci_fft::{FftOptions, dct_iv, dst_iv};

fn naive_dct4(x: &[f64]) -> Vec<f64> {
    let n = x.len();
    (0..n)
        .map(|k| {
            2.0 * x.iter().enumerate().fold(0.0, |acc, (m, &xm)| {
                acc + xm
                    * (std::f64::consts::PI * (2.0 * m as f64 + 1.0) * (2.0 * k as f64 + 1.0)
                        / (4.0 * n as f64))
                        .cos()
            })
        })
        .collect()
}

fn naive_dst4(x: &[f64]) -> Vec<f64> {
    let n = x.len();
    (0..n)
        .map(|k| {
            2.0 * x.iter().enumerate().fold(0.0, |acc, (m, &xm)| {
                acc + xm
                    * (std::f64::consts::PI * (2.0 * m as f64 + 1.0) * (2.0 * k as f64 + 1.0)
                        / (4.0 * n as f64))
                        .sin()
            })
        })
        .collect()
}

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
    for &n in &[2usize, 3, 4, 5, 8, 12, 16, 17, 24, 32, 64, 128, 360, 1000] {
        let x = signal(n, n as u64 * 5147 + 13);
        let c = dct_iv(&x, &opts).expect("dct_iv");
        let s = dst_iv(&x, &opts).expect("dst_iv");
        let wc = naive_dct4(&x);
        let ws = naive_dst4(&x);
        let ec = c.iter().zip(&wc).map(|(a, b)| (a - b).abs()).fold(0.0f64, f64::max);
        let es = s.iter().zip(&ws).map(|(a, b)| (a - b).abs()).fold(0.0f64, f64::max);
        worst = worst.max(ec).max(es);
        println!("n={n:>4} dct4_err={ec:.3e} dst4_err={es:.3e}");
    }
    println!("===GOLDEN_PAYLOAD_END===");
    println!("worst Type-IV err vs naive = {worst:.3e} (parity tol 1e-9)");
    assert!(worst < 1e-9, "Type-IV transform exceeds parity tolerance");

    for &n in &[512usize, 1000, 2048, 4096, 10000, 16384] {
        let x = signal(n, 99);
        let reps = 500;
        let _ = dct_iv(&x, &opts).unwrap();
        macro_rules! time {
            ($name:expr, $f:path) => {{
                let t0 = Instant::now();
                let mut acc = 0.0;
                for _ in 0..reps {
                    let r = $f(black_box(&x), &opts).unwrap();
                    acc += r[r.len() / 2];
                }
                println!("{:<7} n={n:>6} {:>10.3?}/call (acc={acc:.3})", $name, t0.elapsed() / reps);
            }};
        }
        time!("dct_iv", dct_iv);
        time!("dst_iv", dst_iv);
    }
}
