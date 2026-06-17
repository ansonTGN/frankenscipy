//! Same-process A/B + parity harness for the entropy/KL/cross-entropy family.
//!
//! The library bodies are vectorized 8-wide with dual lane-accumulators; this
//! binary holds verbatim SCALAR references and times BOTH back-to-back in one
//! process (interleaved) so the ratio is free of the ~2x cross-worker variance
//! that separate compile/run pairs suffer. The SIMD `ln` is bit-identical to
//! scalar `f64::ln` (0 ULP), so the only deviation from the scalar fold is the
//! harmless sum lane-order — the PARITY block prints both to 17 digits to prove
//! it stays ~1 ULP, and that the edge cases (q==0⇒inf, p<0, empty, total==0)
//! match exactly. Run: `cargo run --profile release-perf -p fsci-stats --bin perf_entropy`.

use std::hint::black_box;
use std::time::Instant;

use fsci_stats::{cross_entropy, entropy, kl_divergence};

// ---- verbatim scalar references (pre-SIMD bodies) ----
// #[inline(never)]: keep them opaque so LLVM can't CSE/hoist the loop-invariant
// call to a constant (the library SIMD fns are cross-crate-opaque already), which
// would make the A/B ratio meaningless.
#[inline(never)]
fn entropy_scalar(pk: &[f64], base: Option<f64>) -> f64 {
    if pk.is_empty() {
        return 0.0;
    }
    if pk.iter().any(|&p| p < 0.0) {
        return f64::NEG_INFINITY;
    }
    let total: f64 = pk.iter().sum();
    if total == 0.0 {
        return f64::NAN;
    }
    let h: f64 = pk
        .iter()
        .map(|&p| {
            let prob = p / total;
            if prob > 0.0 { -prob * prob.ln() } else { 0.0 }
        })
        .sum();
    match base {
        Some(b) => h / b.ln(),
        None => h,
    }
}

#[inline(never)]
fn kl_scalar(pk: &[f64], qk: &[f64], base: Option<f64>) -> f64 {
    if pk.len() != qk.len() || pk.is_empty() {
        return f64::NAN;
    }
    if pk.iter().any(|&p| p < 0.0) || qk.iter().any(|&q| q < 0.0) {
        return f64::NAN;
    }
    let sum_p: f64 = pk.iter().sum();
    let sum_q: f64 = qk.iter().sum();
    if sum_p == 0.0 || sum_q == 0.0 {
        return f64::NAN;
    }
    let mut kl = 0.0;
    for (&p, &q) in pk.iter().zip(qk) {
        let pi = p / sum_p;
        let qi = q / sum_q;
        if pi > 0.0 {
            if qi == 0.0 {
                return f64::INFINITY;
            }
            kl += pi * (pi / qi).ln();
        }
    }
    match base {
        Some(b) => kl / b.ln(),
        None => kl,
    }
}

#[inline(never)]
fn ce_scalar(pk: &[f64], qk: &[f64], base: Option<f64>) -> f64 {
    if pk.len() != qk.len() || pk.is_empty() {
        return f64::NAN;
    }
    if pk.iter().any(|&p| p < 0.0) || qk.iter().any(|&q| q < 0.0) {
        return f64::NAN;
    }
    let sum_p: f64 = pk.iter().sum();
    let sum_q: f64 = qk.iter().sum();
    if sum_p == 0.0 || sum_q == 0.0 {
        return f64::NAN;
    }
    let mut ce = 0.0;
    for (&p, &q) in pk.iter().zip(qk) {
        let pi = p / sum_p;
        let qi = q / sum_q;
        if pi > 0.0 {
            if qi == 0.0 {
                return f64::INFINITY;
            }
            ce -= pi * qi.ln();
        }
    }
    match base {
        Some(b) => ce / b.ln(),
        None => ce,
    }
}

fn lcg(s: &mut u64) -> f64 {
    *s = s
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);
    (*s >> 11) as f64 / (1u64 << 53) as f64
}

fn probs(n: usize, seed: u64) -> Vec<f64> {
    let mut s = seed;
    (0..n).map(|_| lcg(&mut s) + 1e-3).collect()
}

fn time<F: FnMut() -> f64>(reps: usize, mut f: F) -> (f64, f64) {
    let mut acc = 0.0;
    let t = Instant::now();
    for _ in 0..reps {
        acc += f();
    }
    (t.elapsed().as_secs_f64() * 1e3 / reps as f64, acc)
}

fn main() {
    // ---- PARITY: SIMD (lib) vs scalar ref, full precision + edge cases ----
    println!("===PARITY_BEGIN===");
    let mut worst = 0.0_f64;
    for &n in &[7usize, 16, 17, 100, 1000, 4096] {
        let p = probs(n, 11 + n as u64);
        let q = probs(n, 99 + n as u64);
        for (name, a, b) in [
            ("entropy", entropy(&p, None), entropy_scalar(&p, None)),
            (
                "entropy_b2",
                entropy(&p, Some(2.0)),
                entropy_scalar(&p, Some(2.0)),
            ),
            ("kl", kl_divergence(&p, &q, None), kl_scalar(&p, &q, None)),
            ("ce", cross_entropy(&p, &q, None), ce_scalar(&p, &q, None)),
        ] {
            let rel = if b != 0.0 {
                ((a - b) / b).abs()
            } else {
                (a - b).abs()
            };
            worst = worst.max(rel);
            println!("{name} n={n}: simd={a:.17e} scalar={b:.17e} rel={rel:.3e}");
        }
    }
    // edge cases must match exactly
    let edges: [(&str, f64, f64); 5] = [
        ("empty", entropy(&[], None), entropy_scalar(&[], None)),
        (
            "neg",
            entropy(&[0.5, -0.1, 0.6], None),
            entropy_scalar(&[0.5, -0.1, 0.6], None),
        ),
        (
            "zero_total",
            entropy(&[0.0, 0.0], None),
            entropy_scalar(&[0.0, 0.0], None),
        ),
        (
            "kl_q0",
            kl_divergence(&[0.3, 0.7], &[0.0, 1.0], None),
            kl_scalar(&[0.3, 0.7], &[0.0, 1.0], None),
        ),
        (
            "ce_q0",
            cross_entropy(&[0.3, 0.7], &[0.0, 1.0], None),
            ce_scalar(&[0.3, 0.7], &[0.0, 1.0], None),
        ),
    ];
    for (name, a, b) in edges {
        let same = a.to_bits() == b.to_bits() || (a.is_nan() && b.is_nan());
        println!("edge {name}: simd={a:.5e} scalar={b:.5e} bit_exact={same}");
    }
    println!("worst_rel={worst:.3e}");
    println!("===PARITY_END===");

    // ---- SAME-PROCESS A/B timing (interleaved) ----
    for &n in &[256usize, 1000, 4096, 16384] {
        let p = probs(n, 7);
        let q = probs(n, 8);
        let reps = (200_000_000 / (n + 1)).clamp(50, 200_000);
        // interleave scalar/simd to share cache + thermal state
        let (e_s, _) = time(reps, || entropy_scalar(black_box(&p), None));
        let (e_v, _) = time(reps, || entropy(black_box(&p), None));
        let (k_s, _) = time(reps, || kl_scalar(black_box(&p), black_box(&q), None));
        let (k_v, _) = time(reps, || kl_divergence(black_box(&p), black_box(&q), None));
        let (c_s, _) = time(reps, || ce_scalar(black_box(&p), black_box(&q), None));
        let (c_v, _) = time(reps, || cross_entropy(black_box(&p), black_box(&q), None));
        println!(
            "n={n}: entropy {e_s:.6}->{e_v:.6}={:.2}x  kl {k_s:.6}->{k_v:.6}={:.2}x  ce {c_s:.6}->{c_v:.6}={:.2}x",
            e_s / e_v,
            k_s / k_v,
            c_s / c_v
        );
    }
}
