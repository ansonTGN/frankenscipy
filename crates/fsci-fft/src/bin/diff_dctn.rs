use fsci_fft::{FftOptions, Normalization, dctn, dstn, idctn, idstn};
fn opts(n: Normalization) -> FftOptions {
    FftOptions {
        normalization: n,
        ..Default::default()
    }
}
fn dump(name: &str, nl: &str, v: &[f64]) {
    for (i, &x) in v.iter().enumerate() {
        println!("{name},{nl},{i},{x:.17e}");
    }
}
fn main() {
    // 3x4 input
    let inp: Vec<f64> = (0..12)
        .map(|k| {
            let t = k as f64;
            (0.6 * t).sin() + 0.2 * t - 0.04 * t * t + 1.1
        })
        .collect();
    let shape = [3usize, 4];
    for (nl, nrm) in [
        ("backward", Normalization::Backward),
        ("ortho", Normalization::Ortho),
        ("forward", Normalization::Forward),
    ] {
        let o = opts(nrm);
        if let Ok(v) = dctn(&inp, &shape, &o) {
            dump("dctn", nl, &v);
        }
        if let Ok(v) = dstn(&inp, &shape, &o) {
            dump("dstn", nl, &v);
        }
        if let Ok(v) = idctn(&inp, &shape, &o) {
            dump("idctn", nl, &v);
        }
        if let Ok(v) = idstn(&inp, &shape, &o) {
            dump("idstn", nl, &v);
        }
    }
}
