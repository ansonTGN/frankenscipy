//! Differential oracle probe: frequency-response + bilinear vs scipy.signal (gitignored).
//! Lines: `name,kind,i,value`. kind in {w, hre, him, b, a}. Inputs match python comparator.
use fsci_signal::{bilinear, freqs, freqz, lfilter_zi, sosfreqz, SosSection};

fn dump_resp(name: &str, w: &[f64], mag: &[f64], phase: &[f64]) {
    for (i, &x) in w.iter().enumerate() {
        println!("{name},w,{i},{x:.17e}");
    }
    for i in 0..mag.len() {
        let (m, p) = (mag[i], phase[i]);
        println!("{name},hre,{i},{:.17e}", m * p.cos());
        println!("{name},him,{i},{:.17e}", m * p.sin());
    }
}
fn dump_vec(name: &str, kind: &str, v: &[f64]) {
    for (i, &x) in v.iter().enumerate() {
        println!("{name},{kind},{i},{x:.17e}");
    }
}

fn main() {
    // butter(4, 0.2) lowpass
    let b = vec![
        0.004824343357716228,
        0.019297373430864913,
        0.02894606014629737,
        0.019297373430864913,
        0.004824343357716228,
    ];
    let a = vec![
        1.0,
        -2.369513007182038,
        2.313988414415881,
        -1.054665405878568,
        0.18737949236818502,
    ];

    if let Ok(r) = freqz(&b, &a, Some(64)) {
        dump_resp("freqz", &r.w, &r.h_mag, &r.h_phase);
    }
    // SOS form of the same filter
    let sos: Vec<SosSection> = vec![
        [
            0.004824343357716228,
            0.009648686715432456,
            0.004824343357716228,
            1.0,
            -1.0485995763626117,
            0.2961403575616696,
        ],
        [1.0, 2.0, 1.0, 1.0, -1.3209134308194264, 0.6327387928852766],
    ];
    if let Ok(r) = sosfreqz(&sos, Some(64)) {
        dump_resp("sosfreqz", &r.w, &r.h_mag, &r.h_phase);
    }

    // analog freqs: H(s) = (s^2 + 0.1 s + 1) / (s^2 + 0.5 s + 1)
    let bs = [1.0, 0.1, 1.0];
    let as_ = [1.0, 0.5, 1.0];
    let w: Vec<f64> = (0..40).map(|i| 0.05 * (i as f64 + 1.0)).collect();
    if let Ok(r) = freqs(&bs, &as_, &w) {
        dump_resp("freqs", &r.w, &r.h_mag, &r.h_phase);
    }

    // bilinear: analog -> digital
    let (bd, ad) = bilinear(&[1.0, 0.0, 1.0], &[1.0, 0.5, 1.0], 10.0);
    dump_vec("bilinear", "b", &bd);
    dump_vec("bilinear", "a", &ad);

    // lfilter_zi
    if let Ok(zi) = lfilter_zi(&b, &a) {
        dump_vec("lfilter_zi", "zi", &zi);
    }
}
