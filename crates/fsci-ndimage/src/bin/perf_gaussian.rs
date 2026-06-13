//! Same-process A/B + byte-identity for gaussian_filter1d: the new slab line walk (library,
//! via convolve1d_along_axis) vs the old 1-D-kernel-into-N-D-`convolve` path (retained
//! reference). Byte-identical (max|dx|=0) — only the per-pixel unravel/alloc overhead is gone.
//! Run: `cargo run --release -p fsci-ndimage --bin perf_gaussian`.

use std::hint::black_box;
use std::time::Instant;

use fsci_ndimage::{BoundaryMode, NdArray, gaussian_filter1d, gaussian_filter1d_via_convolve_ref};

fn time<F: FnMut()>(reps: usize, mut f: F) -> f64 {
    let t = Instant::now();
    for _ in 0..reps {
        f();
    }
    t.elapsed().as_secs_f64() * 1e3 / reps as f64
}

fn digest(v: &[f64]) -> u64 {
    v.iter().fold(1469598103934665603u64, |h, &x| {
        (h ^ x.to_bits()).wrapping_mul(1099511628211)
    })
}

fn main() {
    let (rows, cols) = (2000usize, 2000usize);
    let data: Vec<f64> = (0..rows * cols)
        .map(|i| ((i * 2654435761usize) as f64 / u32::MAX as f64).fract() - 0.5)
        .collect();
    let arr = NdArray::new(data, vec![rows, cols]).unwrap();

    for axis in [1usize, 0usize] {
        println!("gaussian_filter1d {rows}x{cols}, axis={axis}, Reflect (vs old convolve path):");
        for &sigma in &[1.0f64, 2.0, 4.0, 8.0] {
            let new = gaussian_filter1d(&arr, sigma, axis, 0, BoundaryMode::Reflect, 0.0).unwrap();
            let old =
                gaussian_filter1d_via_convolve_ref(&arr, sigma, axis, 0, BoundaryMode::Reflect, 0.0)
                    .unwrap();
            let bitident = digest(&new.data) == digest(&old.data);
            let reps = 30usize;
            let t_new = time(reps, || {
                black_box(
                    gaussian_filter1d(black_box(&arr), sigma, axis, 0, BoundaryMode::Reflect, 0.0)
                        .unwrap(),
                );
            });
            let t_old = time(reps, || {
                black_box(
                    gaussian_filter1d_via_convolve_ref(
                        black_box(&arr),
                        sigma,
                        axis,
                        0,
                        BoundaryMode::Reflect,
                        0.0,
                    )
                    .unwrap(),
                );
            });
            println!(
                "  sigma={sigma:>4}: old={t_old:>9.4}ms  new={t_new:>8.4}ms  speedup={:>6.2}x  bit_identical={bitident}",
                t_old / t_new
            );
        }
    }
}
