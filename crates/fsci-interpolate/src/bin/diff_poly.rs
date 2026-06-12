//! Differential oracle probe: polynomial helpers + pade vs numpy/scipy (gitignored).
use fsci_interpolate as ip;

fn vec_line(name: &str, v: &[f64]) {
    let s: Vec<String> = v.iter().map(|x| format!("{x:.17e}")).collect();
    println!("{name}|{}", s.join(";"));
}

fn main() {
    // polyfit: fit cubic-ish data
    let x: Vec<f64> = vec![0.0, 0.5, 1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0];
    let y: Vec<f64> = x
        .iter()
        .map(|&t| 1.0 - 2.0 * t + 0.5 * t * t + 0.1 * t * t * t)
        .collect();
    if let Ok(c) = ip::polyfit(&x, &y, 3) {
        vec_line("polyfit3", &c);
    }
    // polyval at several points (descending coeffs)
    let coeffs = vec![0.1, 0.5, -2.0, 1.0]; // 0.1 x^3 + 0.5 x^2 - 2 x + 1
    let pv: Vec<f64> = [0.3_f64, 1.2, 2.7, -1.5]
        .iter()
        .map(|&t| ip::polyval(&coeffs, t))
        .collect();
    vec_line("polyval", &pv);
    // polyder / polyint
    vec_line("polyder1", &ip::polyder(&coeffs, 1));
    vec_line("polyder2", &ip::polyder(&coeffs, 2));
    vec_line("polyint1", &ip::polyint(&coeffs, 1, 0.0));
    vec_line("polyint2k", &ip::polyint(&coeffs, 2, 3.0));
    // polyroots of (x-1)(x-2)(x+3) = x^3 - 7x + 6  -> coeffs [1,0,-7,6]
    let mut r = ip::polyroots(&[1.0, 0.0, -7.0, 6.0]);
    r.sort_by(|a, b| a.partial_cmp(b).unwrap());
    vec_line("polyroots", &r);
    // polymul (descending)
    vec_line("polymul", &ip::polymul(&[1.0, 2.0], &[1.0, -3.0, 2.0]));
    // pade: exp(x) Taylor -> [1, 1, 1/2, 1/6, 1/24, 1/120]
    let taylor = vec![1.0, 1.0, 0.5, 1.0 / 6.0, 1.0 / 24.0, 1.0 / 120.0];
    if let Ok((p, q)) = ip::pade(&taylor, 2, 2) {
        vec_line("pade_p", &p);
        vec_line("pade_q", &q);
    }
    if let Ok((p, q)) = ip::pade(&taylor, 3, 2) {
        vec_line("pade32_p", &p);
        vec_line("pade32_q", &q);
    }
}
