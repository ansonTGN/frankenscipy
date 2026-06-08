//! Timing + correctness harness for the Bartels-Stewart upper-triangular fast
//! path in `solve_sylvester` / `solve_continuous_lyapunov`.
//!
//! When the Schur factor `T_A` is strictly upper triangular (A has only real
//! eigenvalues — the common case), each shifted 1×1-block system `(T_A + s·I)`
//! is upper triangular and is now solved by O(m²) back-substitution instead of a
//! general O(m³) LU, making the column sweep O(n·m²) instead of O(n·m³).
//!
//! Parity is tolerance-based (this solver is already conformance-parity at 1e-9):
//! an upper-triangular system needs no pivoting, so LU reduces to the same
//! back-substitution. We PROVE correctness independently via the residual
//! ‖A X + X Aᵀ − Q‖_max, which must stay ~0, and dump a solution checksum +
//! residual for fixed small cases (compare across the stashed pre-change build).
//! Run: `cargo run --profile release-perf -p fsci-linalg --bin perf_sylvester_tritriangular`.
#![allow(clippy::needless_range_loop)]

use std::hint::black_box;
use std::time::Instant;

use fsci_linalg::{DecompOptions, solve_continuous_lyapunov};

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

// Upper-triangular A with distinct negative diagonal => all-real eigenvalues, so
// the Schur factor T_A is strictly upper triangular and the fast path triggers.
// Q is symmetric. Solvable since λ_i + λ_j < 0 ≠ 0.
fn problem(n: usize, seed: u64) -> (Vec<Vec<f64>>, Vec<Vec<f64>>) {
    let mut g = Lcg(seed);
    let mut a = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in 0..n {
            if i == j {
                a[i][j] = -(i as f64 + 1.0) - g.unit();
            } else if j > i {
                a[i][j] = g.unit() * 2.0 - 1.0;
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

// max |A X + X Aᵀ − Q|
fn residual_max(a: &[Vec<f64>], x: &[Vec<f64>], q: &[Vec<f64>]) -> f64 {
    let n = a.len();
    // AX
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
    // XAᵀ
    let mut xat = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in 0..n {
            let mut s = 0.0;
            for k in 0..n {
                s += x[i][k] * a[j][k];
            }
            xat[i][j] = s;
        }
    }
    let mut m = 0.0f64;
    for i in 0..n {
        for j in 0..n {
            m = m.max((ax[i][j] + xat[i][j] - q[i][j]).abs());
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
        let x = solve_continuous_lyapunov(&a, &q, DecompOptions::default()).expect("lyap");
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
        let _ = solve_continuous_lyapunov(&a, &q, DecompOptions::default());
        let t0 = Instant::now();
        let mut acc = 0.0;
        for _ in 0..reps {
            let x =
                solve_continuous_lyapunov(black_box(&a), black_box(&q), DecompOptions::default())
                    .unwrap();
            acc += x[0][0];
        }
        let res = residual_max(
            &a,
            &solve_continuous_lyapunov(&a, &q, DecompOptions::default()).unwrap(),
            &q,
        );
        println!(
            "n={n:>4}  {:>10.3?}/solve  residual_max={res:.2e} (acc={acc:.6})",
            t0.elapsed() / reps
        );
    }
}
