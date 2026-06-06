//! Correctness + timing harness for DCT-I (`dct_i`) and DST-I (`dst_i`), which
//! now transform the real symmetric/antisymmetric extension with the real-FFT
//! pack (an (N∓1)-point complex FFT) instead of a full (2N∓2)-point complex FFT.
//! Run: `cargo run --release -p fsci-fft --bin perf_dct1`.

use std::hint::black_box;
use std::time::Instant;

use fsci_fft::{FftOptions, dct_i, dst_i};

fn naive_dct1(x: &[f64]) -> Vec<f64> {
    let n = x.len();
    let nm1 = (n - 1) as f64;
    (0..n)
        .map(|k| {
            let mut acc = x[0] + (if k % 2 == 0 { 1.0 } else { -1.0 }) * x[n - 1];
            for (m, &xm) in x.iter().enumerate().take(n - 1).skip(1) {
                acc += 2.0 * xm * (std::f64::consts::PI * m as f64 * k as f64 / nm1).cos();
            }
            acc
        })
        .collect()
}

fn naive_dst1(x: &[f64]) -> Vec<f64> {
    let n = x.len();
    let np1 = (n + 1) as f64;
    (0..n)
        .map(|k| {
            2.0 * x.iter().enumerate().fold(0.0, |a, (m, &xm)| {
                a + xm
                    * (std::f64::consts::PI * (m as f64 + 1.0) * (k as f64 + 1.0) / np1).sin()
            })
        })
        .collect()
}

fn lcg(s: &mut u64) -> f64 {
    *s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
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
    for &n in &[2usize, 3, 4, 5, 8, 12, 16, 17, 33, 64, 128, 361, 1000] {
        let x = signal(n, n as u64 * 4099 + 17);
        let c = dct_i(&x, &opts).expect("dct_i");
        let s = dst_i(&x, &opts).expect("dst_i");
        let ec = c.iter().zip(&naive_dct1(&x)).map(|(a, b)| (a - b).abs()).fold(0.0f64, f64::max);
        let es = s.iter().zip(&naive_dst1(&x)).map(|(a, b)| (a - b).abs()).fold(0.0f64, f64::max);
        worst = worst.max(ec).max(es);
        println!("n={n:>4} dct1_err={ec:.3e} dst1_err={es:.3e}");
    }
    println!("===GOLDEN_PAYLOAD_END===");
    println!("worst Type-I err vs naive = {worst:.3e} (parity tol 1e-9)");
    assert!(worst < 1e-9, "Type-I transform exceeds parity tolerance");

    for &n in &[513usize, 1025, 2049, 4097, 8193, 16385] {
        let x = signal(n, 99);
        let reps = 500;
        let _ = dct_i(&x, &opts).unwrap();
        macro_rules! time {
            ($name:expr, $f:path) => {{
                let t0 = Instant::now();
                let mut acc = 0.0;
                for _ in 0..reps {
                    let r = $f(black_box(&x), &opts).unwrap();
                    acc += r[r.len() / 2];
                }
                println!("{:<6} n={n:>6} {:>10.3?}/call (acc={acc:.3})", $name, t0.elapsed() / reps);
            }};
        }
        time!("dct_i", dct_i);
        time!("dst_i", dst_i);
    }
}
