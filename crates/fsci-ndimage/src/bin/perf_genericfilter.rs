//! Same-process A/B + byte-identity for generic_filter: the new interior flat-gather path
//! (library) vs the old per-pixel alloc-gather reference (parallel, matching the old code).
//! The footprint is gathered in the same flat order and passed to the same closure, so
//! byte-identical. Run: `cargo run --release -p fsci-ndimage --bin perf_genericfilter`.

use std::hint::black_box;
use std::time::Instant;

use fsci_ndimage::{BoundaryMode, NdArray, generic_filter, generic_filter_perpixel_ref};

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

// A cheap reducer (range = max - min) so the per-pixel gather cost is exposed.
fn range_fn(w: &[f64]) -> f64 {
    let mut mn = f64::INFINITY;
    let mut mx = f64::NEG_INFINITY;
    for &v in w {
        if v < mn {
            mn = v;
        }
        if v > mx {
            mx = v;
        }
    }
    mx - mn
}

fn main() {
    let (rows, cols) = (1200usize, 1200usize);
    let data: Vec<f64> = (0..rows * cols)
        .map(|i| ((i * 2654435761usize) as f64 / u32::MAX as f64).fract() - 0.5)
        .collect();
    let arr = NdArray::new(data, vec![rows, cols]).unwrap();

    for &size in &[3usize, 5, 7] {
        let new = generic_filter(&arr, range_fn, size, BoundaryMode::Reflect, 0.0).unwrap();
        let old = generic_filter_perpixel_ref(&arr, range_fn, size, BoundaryMode::Reflect, 0.0);
        let bit = digest(&new.data) == digest(&old.data);
        let reps = 12usize;
        let t_new = time(reps, || {
            black_box(
                generic_filter(black_box(&arr), range_fn, size, BoundaryMode::Reflect, 0.0)
                    .unwrap(),
            );
        });
        let t_old = time(reps, || {
            black_box(generic_filter_perpixel_ref(
                black_box(&arr),
                range_fn,
                size,
                BoundaryMode::Reflect,
                0.0,
            ));
        });
        println!(
            "generic_filter {rows}x{cols} size={size} (footprint {}): old={t_old:>9.4}ms  new={t_new:>8.4}ms  speedup={:>6.2}x  bit_identical={bit}",
            size * size,
            t_old / t_new
        );
    }
}
