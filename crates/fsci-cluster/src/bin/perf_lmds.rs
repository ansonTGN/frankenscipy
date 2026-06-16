// Correctness + A/B for landmark_mds (eig of the m×m landmark block + triangulate all n
// points) vs classical_mds (randomized eig of the full n×n double-centered matrix). Both
// take the same distance matrix and must recover the input pairwise distances; the speedup
// is the wall-clock ratio.
use fsci_cluster::{classical_mds, landmark_mds};
use std::hint::black_box;
use std::time::Instant;

fn max_distance_err(emb: &[Vec<f64>], dist: &[Vec<f64>]) -> f64 {
    let n = emb.len();
    let k = emb[0].len();
    let mut maxerr = 0.0f64;
    for i in 0..n {
        for j in 0..n {
            let de: f64 = (0..k)
                .map(|t| (emb[i][t] - emb[j][t]).powi(2))
                .sum::<f64>()
                .sqrt();
            maxerr = maxerr.max((de - dist[i][j]).abs());
        }
    }
    maxerr
}

fn main() {
    let n = 2500usize;
    let r = 6usize;
    let k = 6usize;
    let m = 40usize; // landmarks
    let mut st: u64 = 0x243f_6a88_85a3_08d3;
    let mut rng = || {
        st = st
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        ((st >> 11) as f64) / (1u64 << 53) as f64 - 0.5
    };
    let pts: Vec<Vec<f64>> = (0..n).map(|_| (0..r).map(|_| rng()).collect()).collect();
    let dist: Vec<Vec<f64>> = (0..n)
        .map(|i| {
            (0..n)
                .map(|j| {
                    (0..r)
                        .map(|t| (pts[i][t] - pts[j][t]).powi(2))
                        .sum::<f64>()
                        .sqrt()
                })
                .collect()
        })
        .collect();

    let lm = landmark_mds(&dist, k, m, 7).expect("landmark_mds");
    println!(
        "landmark_mds distance_maxerr={:.3e}  (m={m} landmarks)",
        max_distance_err(&lm.embedding, &dist)
    );

    let trials = 3;
    let mut tl = Vec::new();
    let mut tc = Vec::new();
    for _ in 0..trials {
        let t = Instant::now();
        black_box(landmark_mds(&dist, k, m, 7).unwrap());
        tl.push(t.elapsed().as_secs_f64());
        let t = Instant::now();
        black_box(classical_mds(&dist, k, 7).unwrap());
        tc.push(t.elapsed().as_secs_f64());
    }
    tl.sort_by(|a, b| a.partial_cmp(b).unwrap());
    tc.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let l_ms = tl[trials / 2] * 1e3;
    let c_ms = tc[trials / 2] * 1e3;
    println!(
        "classical_mds (full n×n) {c_ms:.2} ms | landmark_mds {l_ms:.2} ms | speedup {:.2}x  (n={n} k={k} m={m})",
        c_ms / l_ms
    );
}
