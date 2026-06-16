//! Timing + tolerance-parity harness for `rfft` on even non-power-of-two lengths.
//!
//! rfft now routes every EVEN N through the pack-two-reals-into-one-complex
//! half-size FFT (previously only powers of two; other even N did a full N-point
//! complex FFT and threw away the redundant half). The result is the same real
//! spectrum up to float reassociation. This prints the spectrum checksum (compare
//! vs the stashed full-FFT build: must agree to ~1e-9 relative) and times the win.
//! Run: `cargo run --profile release-perf -p fsci-fft --bin perf_rfft_even`.

use std::hint::black_box;
use std::time::Instant;

use fsci_fft::{FftOptions, rfft};

fn lcg(s: &mut u64) -> f64 {
    *s = s
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);
    (*s >> 11) as f64 / (1u64 << 53) as f64 * 2.0 - 1.0
}

fn signal(n: usize, seed: u64) -> Vec<f64> {
    let mut s = seed;
    (0..n).map(|_| lcg(&mut s)).collect()
}

fn checksum(spec: &[(f64, f64)]) -> (f64, f64) {
    let mut re = 0.0;
    let mut im = 0.0;
    for &(r, i) in spec {
        re += r.abs();
        im += i.abs();
    }
    (re, im)
}

fn main() {
    let opts = FftOptions::default();

    println!("===PARITY_PAYLOAD_BEGIN===");
    // Even non-pow2 (uses new half-size path), an odd control, a pow2 control.
    for &n in &[6usize, 100, 1000, 4096, 4095, 22050] {
        let x = signal(n, 7);
        let (re, im) = checksum(&rfft(&x, &opts).unwrap());
        println!("n={n} re={re:.10e} im={im:.10e}");
    }
    println!("===PARITY_PAYLOAD_END===");

    for &n in &[1000usize, 22050, 44100, 48000] {
        let x = signal(n, 7);
        let reps = 50;
        let _ = rfft(&x, &opts).unwrap();
        let t0 = Instant::now();
        let mut acc = 0.0;
        for _ in 0..reps {
            let spec = rfft(black_box(&x), &opts).unwrap();
            acc += spec[n / 4].0;
        }
        println!("n={n}  {:>10.3?}/call (acc={acc:.6})", t0.elapsed() / reps);
    }
}
