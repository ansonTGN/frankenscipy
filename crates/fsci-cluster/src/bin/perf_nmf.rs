// Correctness + A/B for NMF: NNDSVD initialization (seeded by randomized_svd) vs the
// classic random initialization. On a non-negative low-rank X both reach a good
// factorization, but NNDSVD converges in far fewer iterations / less time.
use fsci_cluster::{NmfInit, nmf};
use std::hint::black_box;
use std::time::Instant;

fn main() {
    let n = 600usize;
    let d = 200usize;
    let r = 10usize;
    let k = 10usize;
    let max_iter = 40usize; // limited budget: init quality dominates convergence here
    let tol = 0.0;
    let mut st: u64 = 0x243f_6a88_85a3_08d3;
    let mut rng = || {
        st = st
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        (st >> 11) as f64 / (1u64 << 53) as f64
    };
    // Non-negative low-rank X = W_true · H_true.
    let wt: Vec<Vec<f64>> = (0..n).map(|_| (0..r).map(|_| rng()).collect()).collect();
    let ht: Vec<Vec<f64>> = (0..r).map(|_| (0..d).map(|_| rng()).collect()).collect();
    let x: Vec<Vec<f64>> = wt
        .iter()
        .map(|wr| {
            (0..d)
                .map(|j| (0..r).map(|t| wr[t] * ht[t][j]).sum())
                .collect()
        })
        .collect();

    let nnd = nmf(&x, k, max_iter, tol, NmfInit::Nndsvd, 7).expect("nmf nndsvd");
    let rnd = nmf(&x, k, max_iter, tol, NmfInit::Random, 7).expect("nmf random");
    // W,H non-negative?
    let nonneg = nnd.w.iter().chain(&nnd.h).flatten().all(|&v| v >= 0.0);
    println!(
        "NNDSVD: iters={} err={:.4e} nonneg={nonneg} | Random: iters={} err={:.4e}",
        nnd.n_iter, nnd.reconstruction_err, rnd.n_iter, rnd.reconstruction_err
    );

    let trials = 3;
    let mut tn = Vec::new();
    let mut tr = Vec::new();
    for _ in 0..trials {
        let t = Instant::now();
        black_box(nmf(&x, k, max_iter, tol, NmfInit::Nndsvd, 7).unwrap());
        tn.push(t.elapsed().as_secs_f64());
        let t = Instant::now();
        black_box(nmf(&x, k, max_iter, tol, NmfInit::Random, 7).unwrap());
        tr.push(t.elapsed().as_secs_f64());
    }
    tn.sort_by(|a, b| a.partial_cmp(b).unwrap());
    tr.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let nn = tn[trials / 2] * 1e3;
    let rr = tr[trials / 2] * 1e3;
    println!(
        "Random-init {rr:.2} ms | NNDSVD-init {nn:.2} ms | speedup {:.2}x  (n={n} d={d} k={k})",
        rr / nn
    );
}
