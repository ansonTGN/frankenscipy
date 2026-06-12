//! Differential oracle probe: fast Hankel transform (fht/ifht/fhtoffset) vs scipy.fft (gitignored).
//! Lines: `name,i,value`. Inputs defined identically in the python comparator.
use fsci_fft::{fht, fhtoffset, ifht, FftOptions};

fn dump(name: &str, v: &[f64]) {
    for (i, &x) in v.iter().enumerate() {
        println!("{name},{i},{x:.17e}");
    }
}

fn main() {
    let o = FftOptions::default();
    // log-spaced input: a power-law-ish profile, n=16
    let n = 16usize;
    let a: Vec<f64> = (0..n)
        .map(|i| {
            let r = (-2.0 + 0.25 * i as f64).exp(); // r in log space
            r.powf(1.5) * (-r).exp() // a(r) = r^1.5 e^{-r}
        })
        .collect();
    let dln = 0.25_f64;

    // fhtoffset for a few (mu, bias)
    for &mu in &[0.0_f64, 0.5, 1.0, 2.0] {
        for &bias in &[0.0_f64, 0.3, -0.5] {
            let off = fhtoffset(dln, mu, 0.0, bias);
            println!("fhtoffset_mu{mu}_b{bias},0,{off:.17e}");
        }
    }

    // fht / ifht across (mu, bias) with offset=0
    for &mu in &[0.0_f64, 0.5, 1.0] {
        for &bias in &[0.0_f64, 0.3] {
            if let Ok(v) = fht(&a, dln, mu, 0.0, bias, &o) {
                dump(&format!("fht_mu{mu}_b{bias}"), &v);
            }
            if let Ok(v) = ifht(&a, dln, mu, 0.0, bias, &o) {
                dump(&format!("ifht_mu{mu}_b{bias}"), &v);
            }
        }
    }

    // fht with the low-ringing offset (the canonical scipy usage)
    for &mu in &[0.0_f64, 1.0] {
        let off = fhtoffset(dln, mu, 0.0, 0.0);
        if let Ok(v) = fht(&a, dln, mu, off, 0.0, &o) {
            dump(&format!("fht_lowring_mu{mu}"), &v);
        }
    }
}
