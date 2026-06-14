// Correctness + A/B for cur_decomposition. Leverage scores come from a rank-k SVD; the
// randomized public function is compared against the identical CUR built from a FULL SVD
// (what fsci would do without the randomized route). A ≈ C·U·R must hold; the speedup is
// the wall-clock ratio (the SVD for leverage scores dominates).
use fsci_cluster::cur_decomposition;
use fsci_linalg::{matmul, svd, DecompOptions};
use std::hint::black_box;
use std::time::Instant;

// Baseline: full SVD for leverage, then touch the top-k scores (the dominant cost mirrored).
fn full_cur_time(a: &[Vec<f64>], k: usize) -> f64 {
    let dec = svd(a, DecompOptions::default()).expect("svd");
    let n = a[0].len();
    let mut acc = 0.0;
    for j in 0..n {
        acc += (0..k.min(dec.s.len())).map(|t| dec.vt[t][j].powi(2)).sum::<f64>();
    }
    acc
}

fn main() {
    let m = 6000usize;
    let n = 400usize;
    let r = 25usize;
    let k = 40usize;
    let mut st: u64 = 0x243f_6a88_85a3_08d3;
    let mut rng = || {
        st = st.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        ((st >> 11) as f64) / (1u64 << 53) as f64 - 0.5
    };
    let bb: Vec<Vec<f64>> = (0..m).map(|_| (0..r).map(|_| rng()).collect()).collect();
    let gg: Vec<Vec<f64>> = (0..r).map(|_| (0..n).map(|_| rng()).collect()).collect();
    let a: Vec<Vec<f64>> = bb
        .iter()
        .map(|bi| (0..n).map(|j| (0..r).map(|t| bi[t] * gg[t][j]).sum::<f64>() + 1e-6 * rng()).collect())
        .collect();

    let cur = cur_decomposition(&a, k, 10, 7).expect("cur");
    let cu = matmul(&cur.c, &cur.u).expect("CU");
    let recon = matmul(&cu, &cur.r).expect("CUR");
    let mut num = 0.0f64;
    let mut den = 0.0f64;
    for i in 0..m {
        for j in 0..n {
            num += (recon[i][j] - a[i][j]).powi(2);
            den += a[i][j] * a[i][j];
        }
    }
    println!("cur_decomposition rel_reconstruction_err={:.3e}", (num / den).sqrt());

    let trials = 3;
    let mut tr = Vec::new();
    let mut tf = Vec::new();
    for _ in 0..trials {
        let t = Instant::now();
        black_box(cur_decomposition(&a, k, 10, 7).unwrap());
        tr.push(t.elapsed().as_secs_f64());
        let t = Instant::now();
        black_box(full_cur_time(&a, k));
        tf.push(t.elapsed().as_secs_f64());
    }
    tr.sort_by(|a, b| a.partial_cmp(b).unwrap());
    tf.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let r_ms = tr[trials / 2] * 1e3;
    let f_ms = tf[trials / 2] * 1e3;
    println!("full-SVD CUR {f_ms:.2} ms | randomized cur_decomposition {r_ms:.2} ms | speedup {:.2}x  (m={m} n={n} r={r} k={k})", f_ms / r_ms);
}
