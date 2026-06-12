//! minimize-method convergence probe vs scipy (gitignored).
use fsci_opt::minimize::{lbfgsb, tnc};
use fsci_opt::{MinimizeOptions, OptimizeMethod, minimize};
use fsci_runtime::RuntimeMode;
fn rosen(x: &[f64]) -> f64 {
    let mut s = 0.0;
    for i in 0..x.len() - 1 {
        s += 100.0 * (x[i + 1] - x[i] * x[i]).powi(2) + (1.0 - x[i]).powi(2);
    }
    s
}
fn run(name: &str, m: OptimizeMethod, f: fn(&[f64]) -> f64, x0: &[f64]) {
    let opts = MinimizeOptions {
        method: Some(m),
        maxiter: Some(5000),
        maxfev: Some(200_000),
        mode: RuntimeMode::Strict,
        ..Default::default()
    };
    match minimize(f, x0, opts) {
        Ok(r) => println!(
            "{name},x,{:?},fun,{:.6e},success,{},nit,{}",
            r.x,
            r.fun.unwrap_or(f64::NAN),
            r.success,
            r.nit
        ),
        Err(e) => println!("{name},ERR,{e:?}"),
    }
}
const BNDS: [(Option<f64>, Option<f64>); 2] = [(Some(0.0), Some(2.0)), (Some(0.0), Some(2.0))];
fn main() {
    use OptimizeMethod::*;
    // 5-D Rosenbrock from standard start.
    let x5 = [-1.2, 1.0, -1.2, 1.0, -1.2];
    for (nm, m) in [
        ("bfgs", Bfgs),
        ("cg", ConjugateGradient),
        ("lbfgsb", LBfgsB),
        ("powell", Powell),
        ("neldermead", NelderMead),
        ("newtoncg", NewtonCg),
    ] {
        run(&format!("r5_{nm}"), m, rosen, &x5);
    }
    // Bounded box: (x-3)^2+(y-3)^2, bounds [0,2]^2 -> optimum (2,2).
    let bf = |x: &[f64]| (x[0] - 3.0).powi(2) + (x[1] - 3.0).powi(2);
    let o = |m| MinimizeOptions {
        method: Some(m),
        bounds: Some(&BNDS),
        maxiter: Some(2000),
        mode: RuntimeMode::Strict,
        ..Default::default()
    };
    if let Ok(r) = lbfgsb(&bf, &[0.0, 0.0], o(OptimizeMethod::LBfgsB), Some(&BNDS)) {
        println!(
            "box_lbfgsb,x,{:?},fun,{:.6e},success,{}",
            r.x,
            r.fun.unwrap_or(f64::NAN),
            r.success
        );
    }
    if let Ok(r) = tnc(&bf, &[0.0, 0.0], o(OptimizeMethod::Tnc)) {
        println!(
            "box_tnc,x,{:?},fun,{:.6e},success,{}",
            r.x,
            r.fun.unwrap_or(f64::NAN),
            r.success
        );
    }
}
