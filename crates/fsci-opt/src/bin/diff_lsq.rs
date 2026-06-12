//! least_squares / curve_fit probe vs scipy (gitignored).
use fsci_opt::curvefit::{CurveFitOptions, LeastSquaresOptions, curve_fit, least_squares};
fn main() {
    // ---- least_squares: classic Rosenbrock as residuals r=[10(x1-x0^2), 1-x0] ----
    let r = |x: &[f64]| vec![10.0 * (x[1] - x[0] * x[0]), 1.0 - x[0]];
    if let Ok(res) = least_squares(r, &[-1.2, 1.0], LeastSquaresOptions::default()) {
        println!(
            "ls_rosen,x,{:?},cost,{:.6e},success,{},nfev,{}",
            res.x, res.cost, res.success, res.nfev
        );
    }
    // ---- curve_fit: y = a*exp(-b*x)+c, true [2.5, 1.3, 0.5] ----
    let xs: Vec<f64> = (0..25).map(|i| i as f64 * 0.2).collect();
    let truth = [2.5_f64, 1.3, 0.5];
    let model = |x: f64, p: &[f64]| p[0] * (-p[1] * x).exp() + p[2];
    let ys: Vec<f64> = xs.iter().map(|&x| model(x, &truth)).collect();
    let opts = CurveFitOptions {
        p0: Some(vec![1.0, 1.0, 1.0]),
        ..Default::default()
    };
    if let Ok(res) = curve_fit(model, &xs, &ys, opts) {
        println!(
            "cf_exp,popt,{:?},success,{}",
            res.popt, res.ls_result.success
        );
    }
    // ---- curve_fit: y = a*sin(b*x+c), true [3, 1.5, 0.5] ----
    let model2 = |x: f64, p: &[f64]| p[0] * (p[1] * x + p[2]).sin();
    let truth2 = [3.0_f64, 1.5, 0.5];
    let ys2: Vec<f64> = xs.iter().map(|&x| model2(x, &truth2)).collect();
    let opts2 = CurveFitOptions {
        p0: Some(vec![2.5, 1.4, 0.4]),
        ..Default::default()
    };
    if let Ok(res) = curve_fit(model2, &xs, &ys2, opts2) {
        println!(
            "cf_sin,popt,{:?},success,{}",
            res.popt, res.ls_result.success
        );
    }
}
