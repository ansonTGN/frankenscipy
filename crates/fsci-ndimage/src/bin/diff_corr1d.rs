//! Differential oracle probe: correlate1d across boundary modes vs scipy.ndimage (gitignored).
use fsci_ndimage::{BoundaryMode, NdArray, correlate1d, correlate1d_with_origin};

fn dump(name: &str, v: &[f64]) {
    let s: Vec<String> = v.iter().map(|x| format!("{x:.17e}")).collect();
    println!("{name}|{}", s.join(";"));
}

fn main() {
    let input = NdArray::new(vec![1.0, 2.0, 4.0, 7.0, 3.0, 9.0, 5.0, 8.0], vec![8]).unwrap();
    let kernels: &[(&str, Vec<f64>)] = &[
        ("k3", vec![1.0, 2.0, -1.0]),
        ("k4", vec![0.5, 1.0, -0.5, 2.0]),
        ("k5", vec![1.0, -2.0, 3.0, -2.0, 1.0]),
    ];
    let modes = [
        ("reflect", BoundaryMode::Reflect),
        ("constant", BoundaryMode::Constant),
        ("nearest", BoundaryMode::Nearest),
        ("wrap", BoundaryMode::Wrap),
    ];
    for (kn, kw) in kernels {
        for (mn, m) in &modes {
            let r = correlate1d(&input, kw, 0, *m, 0.5).unwrap();
            dump(&format!("corr_{kn}_{mn}"), &r.data);
        }
    }
    // origin shifts with reflect for odd kernel
    for origin in [-1i64, 1] {
        let r = correlate1d_with_origin(
            &input,
            &[1.0, 2.0, -1.0],
            0,
            BoundaryMode::Reflect,
            0.0,
            origin,
        )
        .unwrap();
        dump(&format!("corr_k3_reflect_o{origin}"), &r.data);
    }
}
