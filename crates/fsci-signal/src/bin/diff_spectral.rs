//! Differential oracle probe: periodogram/welch/csd vs scipy.signal (gitignored).
use fsci_signal::{csd, periodogram, welch};

fn dump(name: &str, v: &[f64]) {
    let s: Vec<String> = v.iter().map(|x| format!("{x:.17e}")).collect();
    println!("{name}|{}", s.join(";"));
}

fn main() {
    let fs = 100.0;
    let n = 256;
    let x: Vec<f64> = (0..n)
        .map(|i| {
            let t = i as f64 / fs;
            (2.0 * std::f64::consts::PI * 8.0 * t).sin()
                + 0.5 * (2.0 * std::f64::consts::PI * 20.0 * t).cos()
                + 0.1 * (i as f64 * 0.3).sin()
        })
        .collect();
    let y: Vec<f64> = (0..n)
        .map(|i| {
            let t = i as f64 / fs;
            (2.0 * std::f64::consts::PI * 8.0 * t + 0.7).sin() + 0.3 * (2.0 * 9.0 * t).cos()
        })
        .collect();

    // periodogram (default boxcar window)
    let p = periodogram(&x, fs, None).unwrap();
    dump("periodogram_f", &p.frequencies);
    dump("periodogram_psd", &p.psd);

    // welch with several configs
    for (label, win, nperseg) in [("hann64", "hann", 64usize), ("hamming128", "hamming", 128)] {
        let w = welch(&x, fs, Some(win), Some(nperseg), None).unwrap();
        dump(&format!("welch_{label}_f"), &w.frequencies);
        dump(&format!("welch_{label}_psd"), &w.psd);
    }

    // csd (cross-spectral density), hann nperseg=64
    let c = csd(&x, &y, fs, Some("hann"), Some(64), None).unwrap();
    dump("csd_f", &c.frequencies);
    let cre: Vec<f64> = c.csd.iter().map(|v| v.0).collect();
    let cim: Vec<f64> = c.csd.iter().map(|v| v.1).collect();
    dump("csd_re", &cre);
    dump("csd_im", &cim);
}
