//! Same-process A/B + byte-identity for rank/median_filter: the new interior flat-gather
//! path (library, parallel) vs the old per-pixel alloc-gather reference (sequential). The
//! footprint is gathered in the same flat order and rank-selected, so byte-identical.
//! Run: `cargo run --release -p fsci-ndimage --bin perf_rankfilter`.

use std::hint::black_box;
use std::time::Instant;

use fsci_ndimage::{BoundaryMode, NdArray, rank_filter, rank_filter_perpixel_ref};

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
    let (rows, cols) = (1200usize, 1200usize);
    let data: Vec<f64> = (0..rows * cols)
        .map(|i| ((i * 2654435761usize) as f64 / u32::MAX as f64).fract() - 0.5)
        .collect();
    let arr = NdArray::new(data, vec![rows, cols]).unwrap();

    for &size in &[3usize, 5, 7, 9] {
        let footprint = size * size;
        let rank = (footprint / 2) as isize; // median
        let new = rank_filter(&arr, rank, size, BoundaryMode::Reflect, 0.0).unwrap();
        let old = rank_filter_perpixel_ref(&arr, size, footprint / 2, BoundaryMode::Reflect, 0.0);
        let bit = digest(&new.data) == digest(&old.data);
        let reps = 12usize;
        let t_new = time(reps, || {
            black_box(
                rank_filter(black_box(&arr), rank, size, BoundaryMode::Reflect, 0.0).unwrap(),
            );
        });
        let t_old = time(reps, || {
            black_box(rank_filter_perpixel_ref(
                black_box(&arr),
                size,
                footprint / 2,
                BoundaryMode::Reflect,
                0.0,
            ));
        });
        println!(
            "median_filter {rows}x{cols} size={size} (footprint {footprint}): old={t_old:>9.4}ms  new={t_new:>8.4}ms  speedup={:>6.2}x  bit_identical={bit}",
            t_old / t_new
        );
    }
}
