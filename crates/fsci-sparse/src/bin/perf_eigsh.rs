//! Timing + parity harness for eigsh: power-iteration-with-deflation -> shared
//! Lanczos/Arnoldi Krylov solver. Builds a symmetric banded sparse matrix and
//! computes the k largest-magnitude eigenpairs. Both methods converge to the same
//! true eigenvalues (tolerance-parity); we print eigenvalues + residual +
//! nmatvec + time (compare across the stashed pre-change build).
//! Run: `cargo run --profile release-perf -p fsci-sparse --bin perf_eigsh`.

use std::time::Instant;

use fsci_sparse::{CooMatrix, CsrMatrix, EigsOptions, FormatConvertible, Shape2D, eigsh};

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

// Symmetric sparse matrix with a WELL-SEPARATED dominant spectrum: the first few
// diagonal entries are large and distinct (planted top eigenvalues), the rest
// small, plus small mirrored off-diagonal bands. This is the regime eigsh targets
// (a clear gap between the wanted extremes and the bulk), where Lanczos converges.
fn symmetric_banded(n: usize, seed: u64) -> CsrMatrix {
    let mut g = Lcg(seed);
    let offsets = [1usize, 4, 17, 53];
    let mut rows = Vec::new();
    let mut cols = Vec::new();
    let mut data = Vec::new();
    for i in 0..n {
        // Well-separated planted top eigenvalues: 100, 88, 76, ... for i<12.
        let diag = if i < 12 {
            100.0 - 12.0 * i as f64
        } else {
            g.unit() // bulk spectrum in [0,1]
        };
        rows.push(i);
        cols.push(i);
        data.push(diag);
        for &off in &offsets {
            if i + off < n {
                let w = (g.unit() * 2.0 - 1.0) * 0.1;
                rows.push(i);
                cols.push(i + off);
                data.push(w);
                rows.push(i + off);
                cols.push(i);
                data.push(w);
            }
        }
    }
    CooMatrix::from_triplets(Shape2D::new(n, n), data, rows, cols, false)
        .unwrap()
        .to_csr()
        .unwrap()
}

fn residual(a: &CsrMatrix, lambda: f64, x: &[f64]) -> f64 {
    // ||A x - lambda x||
    let indptr = a.indptr();
    let indices = a.indices();
    let data = a.data();
    let mut s = 0.0;
    for i in 0..a.shape().rows {
        let mut ax = 0.0;
        for idx in indptr[i]..indptr[i + 1] {
            ax += data[idx] * x[indices[idx]];
        }
        s += (ax - lambda * x[i]).powi(2);
    }
    s.sqrt()
}

fn main() {
    for &(n, k) in &[(2000usize, 6usize), (8000, 6), (20000, 8)] {
        let a = symmetric_banded(n, 0x1234 ^ n as u64);
        let opt = EigsOptions::default();
        let t0 = Instant::now();
        let r = eigsh(&a, k, opt).expect("eigsh");
        let dt = t0.elapsed();
        let mut evs = r.eigenvalues.clone();
        evs.sort_by(|a, b| b.abs().total_cmp(&a.abs()));
        let max_resid = r
            .eigenvalues
            .iter()
            .zip(r.eigenvectors.iter())
            .map(|(&l, x)| residual(&a, l, x))
            .fold(0.0_f64, f64::max);
        println!(
            "n={n:>6} k={k} {:>10.3?}  nmatvec={:>5} conv={} top3=[{:.6},{:.6},{:.6}] max_resid={max_resid:.2e}",
            dt,
            r.nmatvec,
            r.converged,
            evs.first().copied().unwrap_or(0.0),
            evs.get(1).copied().unwrap_or(0.0),
            evs.get(2).copied().unwrap_or(0.0),
        );
    }
}
