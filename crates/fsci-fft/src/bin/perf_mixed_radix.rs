//! Correctness + timing harness for the mixed-radix Cooley-Tukey path that now
//! serves composite non-power-of-2 FFT lengths (previously all of these routed
//! through Bluestein on a size `next_pow2(2n-1)` convolution).
//!
//! Correctness is checked against a naive O(n²) DFT ground truth (max abs error
//! must be ~machine eps · n, well under the 1e-9 scipy-parity tolerance). The
//! same `fft` public API is timed beside an in-binary copy of the legacy
//! mixed-radix split rule, so the keep/reject ratio is same-process and
//! same-worker.
//! Run: `cargo run --release -p fsci-fft --bin perf_mixed_radix`.

use std::collections::HashMap;
use std::hint::black_box;
use std::sync::{Arc, OnceLock, RwLock};
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
        let _ = legacy_fft(&x);

        let t0 = Instant::now();
        let mut current_acc = 0.0;
        for _ in 0..reps {
            let r = fft(black_box(&x), &opts).unwrap();
            current_acc += r[r.len() / 2].0;
        }
        let current_dt = t0.elapsed() / reps;

        let t0 = Instant::now();
        let mut legacy_acc = 0.0;
        for _ in 0..reps {
            let r = legacy_fft(black_box(&x));
            legacy_acc += r[r.len() / 2].0;
        }
        let legacy_dt = t0.elapsed() / reps;

        let factored = factor_string(n);
        let speedup = legacy_dt.as_secs_f64() / current_dt.as_secs_f64();
        println!(
            "n={n:>6} ({factored:<14}) current={current_dt:>10.3?}/call legacy={legacy_dt:>10.3?}/call speedup={speedup:>5.2}x (acc={current_acc:.3}/{legacy_acc:.3})"
        );
    }
}

fn legacy_fft(input: &[Complex64]) -> Vec<Complex64> {
    let n = input.len();
    let mut out = vec![(0.0, 0.0); n];
    legacy_mixed_radix_fft(input, 0, 1, &mut out, n);
    out
}

fn legacy_mixed_radix_fft(
    src: &[Complex64],
    base: usize,
    stride: usize,
    out: &mut [Complex64],
    n: usize,
) {
    let p = if n.is_multiple_of(4) {
        4
    } else {
        smallest_prime_factor(n)
    };

    if p == n {
        let tw = legacy_twiddles(n);
        for (k, slot) in out.iter_mut().enumerate().take(n) {
            let mut acc = (0.0, 0.0);
            for t in 0..n {
                acc = complex_add(acc, complex_mul(src[base + t * stride], tw[(t * k) % n]));
            }
            *slot = acc;
        }
        return;
    }

    let m = n / p;
    for (j, block) in out.chunks_mut(m).enumerate().take(p) {
        legacy_mixed_radix_fft(src, base + j * stride, stride * p, block, m);
    }

    let twn = legacy_twiddles(n);
    if p == 4 {
        for r in 0..m {
            let t0 = out[r];
            let t1 = complex_mul(out[m + r], twn[r % n]);
            let t2 = complex_mul(out[2 * m + r], twn[(2 * r) % n]);
            let t3 = complex_mul(out[3 * m + r], twn[(3 * r) % n]);
            let a02 = complex_add(t0, t2);
            let b02 = complex_sub(t0, t2);
            let a13 = complex_add(t1, t3);
            let b13 = complex_sub(t1, t3);
            let rot = (b13.1, -b13.0);
            out[r] = complex_add(a02, a13);
            out[m + r] = complex_add(b02, rot);
            out[2 * m + r] = complex_sub(a02, a13);
            out[3 * m + r] = complex_sub(b02, rot);
        }
    } else if p == 2 {
        for r in 0..m {
            let a = out[r];
            let b = complex_mul(out[m + r], twn[r % n]);
            out[r] = complex_add(a, b);
            out[m + r] = complex_sub(a, b);
        }
    } else if p == 3 {
        const S3: f64 = 0.866_025_403_784_438_6;
        for r in 0..m {
            let t0 = out[r];
            let t1 = complex_mul(out[m + r], twn[r % n]);
            let t2 = complex_mul(out[2 * m + r], twn[(2 * r) % n]);
            let psum = complex_add(t1, t2);
            let pdif = complex_sub(t1, t2);
            let a = (t0.0 - 0.5 * psum.0, t0.1 - 0.5 * psum.1);
            out[r] = complex_add(t0, psum);
            out[m + r] = (a.0 + S3 * pdif.1, a.1 - S3 * pdif.0);
            out[2 * m + r] = (a.0 - S3 * pdif.1, a.1 + S3 * pdif.0);
        }
    } else if p == 5 {
        const C1: f64 = 0.309_016_994_374_947_45;
        const C2: f64 = -0.809_016_994_374_947_4;
        const S1: f64 = 0.951_056_516_295_153_6;
        const S2: f64 = 0.587_785_252_292_473_1;
        for r in 0..m {
            let t0 = out[r];
            let t1 = complex_mul(out[m + r], twn[r % n]);
            let t2 = complex_mul(out[2 * m + r], twn[(2 * r) % n]);
            let t3 = complex_mul(out[3 * m + r], twn[(3 * r) % n]);
            let t4 = complex_mul(out[4 * m + r], twn[(4 * r) % n]);
            let t1p4 = complex_add(t1, t4);
            let t1m4 = complex_sub(t1, t4);
            let t2p3 = complex_add(t2, t3);
            let t2m3 = complex_sub(t2, t3);
            let a1 = (
                t0.0 + C1 * t1p4.0 + C2 * t2p3.0,
                t0.1 + C1 * t1p4.1 + C2 * t2p3.1,
            );
            let a2 = (
                t0.0 + C2 * t1p4.0 + C1 * t2p3.0,
                t0.1 + C2 * t1p4.1 + C1 * t2p3.1,
            );
            let b1 = (S1 * t1m4.0 + S2 * t2m3.0, S1 * t1m4.1 + S2 * t2m3.1);
            let b2 = (S2 * t1m4.0 - S1 * t2m3.0, S2 * t1m4.1 - S1 * t2m3.1);
            out[r] = (t0.0 + t1p4.0 + t2p3.0, t0.1 + t1p4.1 + t2p3.1);
            out[m + r] = (a1.0 + b1.1, a1.1 - b1.0);
            out[2 * m + r] = (a2.0 + b2.1, a2.1 - b2.0);
            out[3 * m + r] = (a2.0 - b2.1, a2.1 + b2.0);
            out[4 * m + r] = (a1.0 - b1.1, a1.1 + b1.0);
        }
    } else {
        let twp = legacy_twiddles(p);
        let mut tmp = vec![(0.0, 0.0); p];
        for r in 0..m {
            for (j, slot) in tmp.iter_mut().enumerate() {
                *slot = complex_mul(out[j * m + r], twn[(j * r) % n]);
            }
            for u in 0..p {
                let mut acc = (0.0, 0.0);
                for (j, &t) in tmp.iter().enumerate() {
                    acc = complex_add(acc, complex_mul(t, twp[(j * u) % p]));
                }
                out[u * m + r] = acc;
            }
        }
    }
}

type TwiddleKey = usize;
type TwiddleTable = Arc<[Complex64]>;
static LEGACY_TWIDDLE_CACHE: OnceLock<RwLock<HashMap<TwiddleKey, TwiddleTable>>> = OnceLock::new();

fn legacy_twiddles(n: usize) -> TwiddleTable {
    let cache = LEGACY_TWIDDLE_CACHE.get_or_init(|| RwLock::new(HashMap::new()));
    if let Some(table) = cache.read().ok().and_then(|guard| guard.get(&n).cloned()) {
        return table;
    }

    let mut table = Vec::with_capacity(n);
    for k in 0..n {
        let angle = -2.0 * std::f64::consts::PI * k as f64 / n as f64;
        table.push((angle.cos(), angle.sin()));
    }
    let table = Arc::<[Complex64]>::from(table);
    if let Ok(mut guard) = cache.write() {
        guard.insert(n, Arc::clone(&table));
    }
    table
}

fn smallest_prime_factor(n: usize) -> usize {
    if n.is_multiple_of(2) {
        return 2;
    }
    let mut f = 3;
    while f * f <= n {
        if n.is_multiple_of(f) {
            return f;
        }
        f += 2;
    }
    n
}

fn complex_add(a: Complex64, b: Complex64) -> Complex64 {
    (a.0 + b.0, a.1 + b.1)
}

fn complex_sub(a: Complex64, b: Complex64) -> Complex64 {
    (a.0 - b.0, a.1 - b.1)
}

fn complex_mul(a: Complex64, b: Complex64) -> Complex64 {
    (a.0 * b.0 - a.1 * b.1, a.0 * b.1 + a.1 * b.0)
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
