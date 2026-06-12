use fsci_integrate::{SolveIvpOptions, SolverKind, ToleranceValue, solve_ivp};
use fsci_runtime::RuntimeMode;

fn main() {
    let mu = 10.0;
    let mut fun = |_t: f64, y: &[f64]| vec![y[1], mu * (1.0 - y[0] * y[0]) * y[1] - y[0]];
    let y0 = [2.0, 0.0];
    for method in [SolverKind::Lsoda, SolverKind::Bdf, SolverKind::Radau] {
        let opts = SolveIvpOptions {
            t_span: (0.0, 20.0),
            y0: &y0,
            method,
            rtol: 1e-6,
            atol: ToleranceValue::Scalar(1e-9),
            mode: RuntimeMode::Strict,
            ..Default::default()
        };
        match solve_ivp(&mut fun, &opts) {
            Ok(r) => {
                let n = r.t.len();
                let last = n.saturating_sub(1);
                println!(
                    "{method:?}: success={} status={} nfev={} steps={} t_final={:.6} y_final=[{:.6},{:.6}] msg={}",
                    r.success,
                    r.status,
                    r.nfev,
                    n,
                    r.t[last],
                    r.y[last][0],
                    r.y[last][1],
                    r.message
                );
            }
            Err(e) => println!("{method:?}: ERR {e:?}"),
        }
    }
}
