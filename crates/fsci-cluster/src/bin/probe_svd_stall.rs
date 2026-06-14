// Characterize the fsci_linalg::svd / eigh stall on rank-deficient / clustered spectra
// (bead frankenscipy-9xrce). Times svd on a few shapes/ranks to see whether the cost tracks
// O(n^3) (genuine) or blows up pathologically on degenerate spectra (a fixable stall).
use fsci_linalg::{eigh, svd, DecompOptions};
use std::hint::black_box;
use std::time::Instant;

fn lowrank(m: usize, n: usize, r: usize, noise: f64, seed: u64) -> Vec<Vec<f64>> {
    let mut st = seed;
    let mut rng = || {
        st = st.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        ((st >> 11) as f64) / (1u64 << 53) as f64 - 0.5
    };
    let b: Vec<Vec<f64>> = (0..m).map(|_| (0..r).map(|_| rng()).collect()).collect();
    let g: Vec<Vec<f64>> = (0..r).map(|_| (0..n).map(|_| rng()).collect()).collect();
    (0..m)
        .map(|i| (0..n).map(|j| (0..r).map(|t| b[i][t] * g[t][j]).sum::<f64>() + noise * rng()).collect())
        .collect()
}

fn time_svd(a: &[Vec<f64>]) -> f64 {
    let t = Instant::now();
    let s = svd(a, DecompOptions::default()).expect("svd");
    black_box(s.s.iter().sum::<f64>());
    t.elapsed().as_secs_f64() * 1e3
}

fn main() {
    // Tall, exactly rank-deficient (the historic 25s case) vs full-rank, same shape.
    for &(m, n, r, noise, tag) in &[
        (800usize, 600usize, 30usize, 0.0f64, "800x600 rank-30 EXACT"),
        (800, 600, 30, 1e-6, "800x600 rank-30 +1e-6 noise"),
        (800, 600, 600, 0.0, "800x600 FULL rank"),
        (600, 600, 30, 0.0, "600x600 rank-30 EXACT square"),
        (600, 600, 600, 0.0, "600x600 FULL rank square"),
    ] {
        let a = lowrank(m, n, r.min(n), noise, 0x1234 + m as u64);
        println!("svd {tag}: {:.1} ms", time_svd(&a));
    }

    // eigh on a rank-deficient symmetric kernel vs full-rank, n=800.
    for &(n, r, tag) in &[(800usize, 30usize, "K=BBᵀ rank-30"), (800, 800, "K full-rank")] {
        let b = lowrank(n, r, r, 0.0, 0xaa + n as u64);
        let k: Vec<Vec<f64>> = (0..n)
            .map(|i| (0..n).map(|j| (0..r).map(|t| b[i][t] * b[j][t]).sum()).collect())
            .collect();
        let t = Instant::now();
        black_box(eigh(&k, DecompOptions::default()).expect("eigh").eigenvalues[0]);
        println!("eigh {tag} (n={n}): {:.1} ms", t.elapsed().as_secs_f64() * 1e3);
    }
}
