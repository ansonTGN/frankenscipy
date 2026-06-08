//! Byte-identity + timing harness for kron (Kronecker product), now parallel over the
//! independent output rows. Each output cell is a single product written exactly once
//! (bijective index map), so the result is bit-identical to the serial quadruple loop.
//! Compare across the stashed serial build.
//! Run: `cargo run --profile release-perf -p fsci-linalg --bin perf_kron`.

use std::hint::black_box;
use std::time::Instant;

use fsci_linalg::kron;

fn lcg(s: &mut u64) -> f64 {
    *s = s
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);
    (*s >> 11) as f64 / (1u64 << 53) as f64
}
fn mat(r: usize, c: usize, seed: u64) -> Vec<Vec<f64>> {
    let mut s = seed;
    (0..r)
        .map(|_| (0..c).map(|_| (lcg(&mut s) - 0.5) * 4.0).collect())
        .collect()
}
fn golden(m: &[Vec<f64>]) -> (usize, usize, u64) {
    let mut acc = 0u64;
    for (i, row) in m.iter().enumerate() {
        for (j, &v) in row.iter().enumerate() {
            acc ^= v.to_bits().rotate_left(((i * 31 + j) % 64) as u32);
        }
    }
    (m.len(), m.first().map_or(0, Vec::len), acc)
}

fn main() {
    println!("===GOLDEN_PAYLOAD_BEGIN===");
    for &(ra, ca, rb, cb) in &[
        (4usize, 3usize, 5usize, 2usize),
        (16, 16, 16, 16),
        (40, 30, 20, 25),
    ] {
        let a = mat(ra, ca, 1);
        let b = mat(rb, cb, 2);
        let (rr, cc, acc) = golden(&kron(&a, &b));
        println!("a={ra}x{ca} b={rb}x{cb} -> {rr}x{cc} xor={acc:016x}");
    }
    println!("===GOLDEN_PAYLOAD_END===");

    for &(ra, ca, rb, cb) in &[(128usize, 128usize, 32usize, 32usize), (256, 64, 32, 32)] {
        let a = mat(ra, ca, 7);
        let b = mat(rb, cb, 8);
        let reps = 5;
        let _ = kron(&a, &b);
        let t0 = Instant::now();
        let mut acc = 0.0;
        for _ in 0..reps {
            acc += kron(black_box(&a), black_box(&b))[0][0];
        }
        let out_rows = ra * rb;
        let out_cols = ca * cb;
        println!(
            "kron {ra}x{ca}⊗{rb}x{cb}=({out_rows}x{out_cols})  {:>10.3?}/call (acc={acc:.6})",
            t0.elapsed() / reps
        );
    }
}
