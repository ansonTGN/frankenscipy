//! Timing + golden-digest harness for the N-D real FFT (`rfftn`), whose
//! rfft-along-last-axis pass transforms each contiguous lane independently.
//!
//! Each lane is an independent real→complex transform writing its own
//! `last/2+1` outputs, so the lane loop parallelizes byte-identically. This
//! dumps an FNV digest of the complex output bits (must be unchanged across the
//! serial/parallel builds) and times the large-array win.
//! Run: `cargo run --release -p fsci-fft --bin perf_rfftn`.

use std::hint::black_box;
use std::time::Instant;

use fsci_fft::{FftOptions, rfftn};

fn lcg(s: &mut u64) -> f64 {
    *s = s
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);
    (*s >> 11) as f64 / (1u64 << 53) as f64 * 2.0 - 1.0
}

fn digest(v: &[(f64, f64)]) -> u64 {
    v.iter().fold(1469598103934665603u64, |h, &(re, im)| {
        let h = (h ^ re.to_bits()).wrapping_mul(1099511628211);
        (h ^ im.to_bits()).wrapping_mul(1099511628211)
    })
}

fn bench(label: &str, shape: &[usize]) {
    let total: usize = shape.iter().product();
    let mut s = 0x0fed_cba9_8765_4321u64;
    let data: Vec<f64> = (0..total).map(|_| lcg(&mut s)).collect();
    let opts = FftOptions::default();

    let out = rfftn(&data, shape, &opts).unwrap();
    let dig = digest(&out);

    let trials = 5;
    let mut t = Vec::with_capacity(trials);
    for _ in 0..trials {
        let t0 = Instant::now();
        black_box(rfftn(&data, shape, &opts).unwrap());
        t.push(t0.elapsed().as_secs_f64());
    }
    t.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!(
        "{label} shape={shape:?} median={:.2} ms  GOLDEN digest={dig:016x}",
        t[trials / 2] * 1e3
    );
}

fn main() {
    // Many lanes along the last axis exercise the parallelized real-FFT pass.
    bench("rfftn 2D", &[4096, 2048]);
    bench("rfftn 3D", &[256, 64, 512]);
}
