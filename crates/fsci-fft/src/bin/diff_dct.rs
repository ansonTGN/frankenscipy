//! Differential oracle probe: DCT/DST family vs scipy.fft (gitignored).
//! Lines: `name,norm,i,value`. Fixed input defined identically in the python comparator.
use fsci_fft::Normalization;
use fsci_fft::{FftOptions, dct, dct_i, dct_iii, dct_iv, dst_i, dst_ii, dst_iii, dst_iv};

fn opts(n: Normalization) -> FftOptions {
    FftOptions {
        normalization: n,
        ..Default::default()
    }
}

fn dump(name: &str, norm: &str, v: &[f64]) {
    for (i, &x) in v.iter().enumerate() {
        println!("{name},{norm},{i},{x:.17e}");
    }
}

fn main() {
    // deterministic non-trivial real input (length 9, odd, to exercise the parity paths)
    let x: Vec<f64> = (0..9)
        .map(|i| {
            let t = i as f64;
            (0.7 * t).sin() + 0.3 * t - 0.05 * t * t + 1.3
        })
        .collect();

    let norms = [
        ("backward", Normalization::Backward),
        ("ortho", Normalization::Ortho),
        ("forward", Normalization::Forward),
    ];

    for (nlabel, nrm) in norms {
        let o = opts(nrm);
        // DCT types 1..4 (dct == type II)
        if let Ok(v) = dct_i(&x, &o) {
            dump("dct1", nlabel, &v);
        }
        if let Ok(v) = dct(&x, &o) {
            dump("dct2", nlabel, &v);
        }
        if let Ok(v) = dct_iii(&x, &o) {
            dump("dct3", nlabel, &v);
        }
        if let Ok(v) = dct_iv(&x, &o) {
            dump("dct4", nlabel, &v);
        }
        // DST types 1..4
        if let Ok(v) = dst_i(&x, &o) {
            dump("dst1", nlabel, &v);
        }
        if let Ok(v) = dst_ii(&x, &o) {
            dump("dst2", nlabel, &v);
        }
        if let Ok(v) = dst_iii(&x, &o) {
            dump("dst3", nlabel, &v);
        }
        if let Ok(v) = dst_iv(&x, &o) {
            dump("dst4", nlabel, &v);
        }
    }
}
