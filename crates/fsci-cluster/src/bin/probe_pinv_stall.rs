// Does pinv/lstsq/solve stall on rank-deficient inputs the way svd() did (9xrce)? They call
// safe_svd directly, bypassing the now-fixed svd() rank-deficient reroute.
use fsci_linalg::{LstsqOptions, PinvOptions, SolveOptions, lstsq, pinv, solve};
use std::hint::black_box;
use std::time::Instant;

fn lowrank(m: usize, n: usize, r: usize, seed: u64) -> Vec<Vec<f64>> {
    let mut st = seed;
    let mut rng = || {
        st = st
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        ((st >> 11) as f64) / (1u64 << 53) as f64 - 0.5
    };
    let b: Vec<Vec<f64>> = (0..m).map(|_| (0..r).map(|_| rng()).collect()).collect();
    let g: Vec<Vec<f64>> = (0..r).map(|_| (0..n).map(|_| rng()).collect()).collect();
    (0..m)
        .map(|i| {
            (0..n)
                .map(|j| (0..r).map(|t| b[i][t] * g[t][j]).sum())
                .collect()
        })
        .collect()
}

fn main() {
    // pinv of a rank-deficient matrix (its whole purpose).
    let a = lowrank(400, 300, 25, 0x77);
    let t = Instant::now();
    let p = pinv(&a, PinvOptions::default());
    println!(
        "pinv 400x300 rank-25: {:.1} ms (ok={})",
        t.elapsed().as_secs_f64() * 1e3,
        p.is_ok()
    );

    // lstsq with a rank-deficient (singular) square system.
    let sq = lowrank(300, 300, 25, 0x88);
    let bvec: Vec<f64> = (0..300).map(|i| (i as f64).sin()).collect();
    let t = Instant::now();
    let l = lstsq(&sq, &bvec, LstsqOptions::default());
    println!(
        "lstsq 300x300 rank-25: {:.1} ms (ok={})",
        t.elapsed().as_secs_f64() * 1e3,
        l.is_ok()
    );

    // solve of a singular system (LU/QR fail -> SVD fallback).
    let t = Instant::now();
    let s = solve(&sq, &bvec, SolveOptions::default());
    println!(
        "solve 300x300 singular: {:.1} ms (ok={})",
        t.elapsed().as_secs_f64() * 1e3,
        s.is_ok()
    );
    black_box((p.is_ok(), l.is_ok(), s.is_ok()));
}
