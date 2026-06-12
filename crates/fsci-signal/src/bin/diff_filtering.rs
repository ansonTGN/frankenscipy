//! Differential oracle probe: lfilter/filtfilt/sosfilt/sosfiltfilt/hilbert/detrend/deconvolve
//! vs scipy.signal (gitignored). Lines: `name|v0;v1;...`.
use fsci_signal::{
    deconvolve, detrend, filtfilt, hilbert, lfilter, sosfilt, sosfiltfilt, DetrendType, SosSection,
};

fn dump(name: &str, v: &[f64]) {
    let s: Vec<String> = v.iter().map(|x| format!("{x:.17e}")).collect();
    println!("{name}|{}", s.join(";"));
}

fn main() {
    // butter(4, 0.2) lowpass coefficients (from scipy).
    let b = [
        0.004824343357716228,
        0.019297373430864913,
        0.02894606014629737,
        0.019297373430864913,
        0.004824343357716228,
    ];
    let a = [
        1.0,
        -2.369513007182038,
        2.313988414415881,
        -1.054665405878568,
        0.18737949236818502,
    ];
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
    // deterministic test signal
    let x: Vec<f64> = (0..40)
        .map(|i| {
            let t = i as f64 * 0.1;
            (2.0 * std::f64::consts::PI * t).sin() + 0.4 * (8.0 * t).cos() + 0.05 * t
        })
        .collect();

    dump("lfilter", &lfilter(&b, &a, &x, None).unwrap());
    dump("filtfilt", &filtfilt(&b, &a, &x).unwrap());
    dump("sosfilt", &sosfilt(&sos, &x).unwrap());
    dump("sosfiltfilt", &sosfiltfilt(&sos, &x).unwrap());

    // hilbert -> analytic signal (real, imag)
    let h = hilbert(&x).unwrap();
    let hr: Vec<f64> = h.iter().map(|c| c.0).collect();
    let hi: Vec<f64> = h.iter().map(|c| c.1).collect();
    dump("hilbert_re", &hr);
    dump("hilbert_im", &hi);

    dump("detrend_linear", &detrend(&x, DetrendType::Linear).unwrap());
    dump("detrend_const", &detrend(&x, DetrendType::Constant).unwrap());

    // deconvolve: x convolved-ish; use a simple divisor
    let sig = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0];
    let div = [1.0, 1.0, 1.0];
    let (q, r) = deconvolve(&sig, &div).unwrap();
    dump("deconv_q", &q);
    dump("deconv_r", &r);
}
