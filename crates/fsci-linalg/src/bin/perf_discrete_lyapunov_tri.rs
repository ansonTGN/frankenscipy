//! Timing + correctness harness for the upper-triangular fast path in
//! `solve_discrete_lyapunov`.
//!
//! The discrete Bartels-Stewart column sweep solves each 1×1 Schur block
//! `(T[j,j]·T − I) y = rhs` where the Schur factor `T` is upper quasi-triangular,
//! using a general O(n³) LU. When `T` is strictly upper triangular (A has only
//! real eigenvalues — the common case) the system is upper triangular and now
//! uses O(n²) back-substitution, making the sweep O(n³) instead of O(n⁴).
//!
//! Parity is tolerance-based (this solver is conformance-parity, not bit-exact):
//! an upper-triangular matrix needs no pivoting so LU reduces to the same
//! back-substitution. We PROVE correctness independently via the residual
//! ‖A X Aᵀ − X + Q‖_max, which must stay ~0.
//! Run: `cargo run --profile release-perf -p fsci-linalg --bin perf_discrete_lyapunov_tri`.
#![allow(clippy::needless_range_loop)]

use std::hint::black_box;
use std::time::Instant;

use fsci_linalg::{DecompOptions, solve_discrete_lyapunov};

struct Lcg(u64);
impl Lcg {
    fn next_u64(&mut self) -> u64 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.0
    }
    fn unit(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }
}

// Upper-triangular A with |diagonal| < 0.9 => all-real eigenvalues, Schur-stable
// (|λ_i λ_j| < 1 ≠ 1), so the Schur factor T is strictly upper triangular and the
// fast path triggers. Q symmetric.
fn problem(n: usize, seed: u64) -> (Vec<Vec<f64>>, Vec<Vec<f64>>) {
    let mut g = Lcg(seed);
    let mut a = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in 0..n {
            if i == j {
                a[i][j] = 0.5 * (g.unit() * 2.0 - 1.0);
            } else if j > i {
                a[i][j] = (g.unit() * 2.0 - 1.0) * 0.1;
            }
        }
    }
    let mut q = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in i..n {
            let v = g.unit() * 2.0 - 1.0;
            q[i][j] = v;
            q[j][i] = v;
        }
    }
    (a, q)
}

// max |A X Aᵀ − X + Q|
fn residual_max(a: &[Vec<f64>], x: &[Vec<f64>], q: &[Vec<f64>]) -> f64 {
    let n = a.len();
    let mut ax = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in 0..n {
            let mut s = 0.0;
            for k in 0..n {
                s += a[i][k] * x[k][j];
            }
            ax[i][j] = s;
        }
    }
    let mut axat = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in 0..n {
            let mut s = 0.0;
            for k in 0..n {
                s += ax[i][k] * a[j][k];
            }
            axat[i][j] = s;
        }
    }
    let mut m = 0.0f64;
    for i in 0..n {
        for j in 0..n {
            m = m.max((axat[i][j] - x[i][j] + q[i][j]).abs());
        }
    }
    m
}

fn checksum(x: &[Vec<f64>]) -> u64 {
    x.iter()
        .flatten()
        .fold(0u64, |h, &v| h.rotate_left(7) ^ v.to_bits())
}

fn main() {
    println!("===GOLDEN_PAYLOAD_BEGIN===");
    for &(n, seed) in &[(6usize, 1u64), (20, 2), (50, 3)] {
        let (a, q) = problem(n, seed);
        let x = solve_discrete_lyapunov(&a, &q, DecompOptions::default()).expect("dlyap");
        println!(
            "n={n} seed={seed} chk={:016x} residual_max={:.3e}",
            checksum(&x),
            residual_max(&a, &x, &q)
        );
    }
    println!("===GOLDEN_PAYLOAD_END===");

    for &n in &[64usize, 128, 256] {
        let (a, q) = problem(n, 7);
        let reps = if n <= 128 { 20 } else { 5 };
        let _ = solve_discrete_lyapunov(&a, &q, DecompOptions::default());
        let t0 = Instant::now();
        let mut acc = 0.0;
        for _ in 0..reps {
            let x = solve_discrete_lyapunov(black_box(&a), black_box(&q), DecompOptions::default())
                .unwrap();
            acc += x[0][0];
        }
        let res = residual_max(
            &a,
            &solve_discrete_lyapunov(&a, &q, DecompOptions::default()).unwrap(),
            &q,
        );
        println!(
            "n={n:>4}  {:>10.3?}/solve  residual_max={res:.2e} (acc={acc:.6})",
            t0.elapsed() / reps
        );
    }
}
