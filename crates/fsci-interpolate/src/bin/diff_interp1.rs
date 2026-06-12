//! 1D interpolators probe vs scipy.interpolate (gitignored).
use fsci_interpolate::{
    akima1d_interpolate, barycentric_interpolate, krogh_interpolate, pade, pchip_interpolate,
};
fn emit(name: &str, v: &[f64]) {
    println!(
        "{name},{}",
        v.iter()
            .map(|x| format!("{x:.17e}"))
            .collect::<Vec<_>>()
            .join(";")
    );
}
fn main() {
    let xi = [0.0, 0.5, 1.3, 2.1, 2.9, 3.6, 4.4, 5.0];
    let yi: Vec<f64> = xi
        .iter()
        .map(|&x: &f64| (0.8 * x).sin() + 0.2 * x * x - 0.5 * x)
        .collect();
    let xq: Vec<f64> = (0..25).map(|i| 0.1 + i as f64 * 0.19).collect();
    emit(
        "barycentric",
        &barycentric_interpolate(&xi, &yi, &xq).unwrap(),
    );
    emit("krogh", &krogh_interpolate(&xi, &yi, &xq).unwrap());
    // pchip/akima need >= 2/5 pts; use a monotone-ish dataset
    let xp = [0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    let yp: Vec<f64> = xp.iter().map(|&x: &f64| x.powf(1.5) + 0.3 * x).collect();
    let xpq: Vec<f64> = (0..30).map(|i| i as f64 * 0.2).collect();
    emit("pchip", &pchip_interpolate(&xp, &yp, &xpq).unwrap());
    emit("akima", &akima1d_interpolate(&xp, &yp, &xpq).unwrap());
    // pade: exp(x) Taylor coeffs [1, 1, 1/2, 1/6, 1/24, 1/120], m=2, n=3
    let tc = [1.0, 1.0, 0.5, 1.0 / 6.0, 1.0 / 24.0, 1.0 / 120.0];
    let (p, q) = pade(&tc, 2, 3).unwrap();
    emit("pade_p", &p);
    emit("pade_q", &q);
}
