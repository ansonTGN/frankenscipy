//! Differential probe: CubicSplineStandalone bc_types vs scipy.interpolate.CubicSpline
use fsci_interpolate::{CubicSplineStandalone, SplineBc};
fn main() {
    let xi: Vec<f64> = vec![0.0, 0.5, 1.1, 1.7, 2.4, 3.0, 3.8, 4.5, 5.2, 6.0];
    let yi: Vec<f64> = xi
        .iter()
        .map(|&x| x.sin() + 0.3 * x * x - 0.5 * x)
        .collect();
    // periodic needs y[0]==y[n-1]; build a separate periodic dataset
    let xp: Vec<f64> = vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    let mut yp: Vec<f64> = xp
        .iter()
        .map(|&x| (x * std::f64::consts::PI / 3.0).sin())
        .collect();
    let n = yp.len();
    yp[n - 1] = yp[0];
    let mut xnew = Vec::new();
    let mut x = 0.1;
    while x < 5.95 {
        xnew.push(x);
        x += 0.13;
    }
    let emit = |name: &str, s: &CubicSplineStandalone, xs: &[f64]| {
        for (i, &q) in xs.iter().enumerate() {
            println!("{name},{i},{:.17e}", s.eval(q));
        }
    };
    let nat = CubicSplineStandalone::new(&xi, &yi, SplineBc::Natural).unwrap();
    emit("natural", &nat, &xnew);
    let nak = CubicSplineStandalone::new(&xi, &yi, SplineBc::NotAKnot).unwrap();
    emit("notaknot", &nak, &xnew);
    let cl = CubicSplineStandalone::new(&xi, &yi, SplineBc::Clamped(0.5, -1.2)).unwrap();
    emit("clamped", &cl, &xnew);
    let per = CubicSplineStandalone::new(&xp, &yp, SplineBc::Periodic).unwrap();
    let mut xnp = Vec::new();
    let mut x = 0.1;
    while x < 5.95 {
        xnp.push(x);
        x += 0.13;
    }
    emit("periodic", &per, &xnp);
    // derivative + integral of not-a-knot (scipy CubicSpline(...).derivative()/integrate)
    let d1 = nak.derivative(1);
    for (i, &q) in xnew.iter().enumerate() {
        println!("nak_d1,{i},{:.17e}", d1.eval(q));
    }
    println!("nak_integ,0,{:.17e}", nak.integrate(0.3, 5.5));
}
