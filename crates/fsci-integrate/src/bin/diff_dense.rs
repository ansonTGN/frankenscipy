//! Dense-output accuracy probe vs analytic + scipy (gitignored).
use fsci_integrate::{SolveIvpOptions, SolverKind, ToleranceValue, solve_ivp};
use fsci_runtime::RuntimeMode;
fn main() {
    // y' = -y, y(0)=1 -> exp(-t). BDF/Radau take large steps as it flattens.
    let teval: Vec<f64> = (1..=40).map(|k| k as f64 * 0.25).collect();
    for method in [SolverKind::Bdf, SolverKind::Radau, SolverKind::Rk45] {
        let opts = SolveIvpOptions {
            t_span: (0.0, 10.0),
            y0: &[1.0],
            method,
            t_eval: Some(&teval),
            rtol: 1e-6,
            atol: ToleranceValue::Scalar(1e-9),
            mode: RuntimeMode::Strict,
            ..Default::default()
        };
        if let Ok(r) = solve_ivp(&mut |_t, y| vec![-y[0]], &opts) {
            let mut maxerr = 0.0_f64;
            for (i, &t) in teval.iter().enumerate() {
                let exact = (-t).exp();
                let e = (r.y[i][0] - exact).abs() / exact.max(1e-300);
                maxerr = maxerr.max(e);
            }
            println!("{method:?},maxrelerr_vs_exact,{maxerr:.3e}");
        }
    }
}
