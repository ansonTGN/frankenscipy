//! Differential oracle probe: signal window functions vs scipy.signal.windows (gitignored).
//! Lines: `name,arg,n,i,value`. Run via release-perf; pipe to python comparator.
use fsci_signal as sig;

fn dump(name: &str, arg: &str, w: &[f64]) {
    let n = w.len();
    for (i, &v) in w.iter().enumerate() {
        println!("{name},{arg},{n},{i},{v:.17e}");
    }
}

fn main() {
    let sizes = [2usize, 3, 8, 16, 31, 64, 101];
    for &n in &sizes {
        dump("hann", "0", &sig::hann(n));
        dump("hamming", "0", &sig::hamming(n));
        dump("blackman", "0", &sig::blackman(n));
        dump("bartlett", "0", &sig::bartlett(n));
        dump("flattop", "0", &sig::flattop(n));
        dump("cosine", "0", &sig::cosine(n));
        dump("parzen", "0", &sig::parzen(n));
        dump("lanczos", "0", &sig::lanczos(n));
        dump("triang", "0", &sig::triang(n));
        dump("barthann", "0", &sig::barthann(n));
        dump("blackmanharris", "0", &sig::blackmanharris(n));
        dump("nuttall", "0", &sig::nuttall_window(n));
        dump("bohman", "0", &sig::bohman_window(n));
        dump("general_hamming", "0.6", &sig::general_hamming(n, 0.6));
        dump("tukey", "0.5", &sig::tukey_window(n, 0.5));
        for &beta in &[2.0_f64, 8.6, 14.0] {
            dump("kaiser", &format!("{beta}"), &sig::kaiser(n, beta));
        }
        for &std in &[1.0_f64, 3.0, 7.0] {
            dump("gaussian", &format!("{std}"), &sig::gaussian(n, std, true));
        }
        for &at in &[50.0_f64, 80.0, 100.0] {
            dump("chebwin", &format!("{at}"), &sig::chebwin(n, at));
        }
    }
}
