// Probe: fsci tanhsinh vs scipy.integrate.tanhsinh on endpoint-singular integrals.
use fsci_integrate::tanhsinh;

fn main() {
    // (label, f, a, b, exact)
    let cases: Vec<(&str, fn(f64) -> f64, f64, f64)> = vec![
        ("1/sqrt(x) on (0,1) = 2", |x| 1.0 / x.sqrt(), 0.0, 1.0),
        ("ln(x) on (0,1) = -1", |x| x.ln(), 0.0, 1.0),
        (
            "1/sqrt(x(1-x)) on (0,1) = pi",
            |x| 1.0 / (x * (1.0 - x)).sqrt(),
            0.0,
            1.0,
        ),
        ("exp(x) on (0,1) = e-1", |x| x.exp(), 0.0, 1.0),
        (
            "cos(x) on (0,pi/2) = 1",
            |x| x.cos(),
            0.0,
            std::f64::consts::FRAC_PI_2,
        ),
        ("x^2 on (-1,2) = 3", |x| x * x, -1.0, 2.0),
        ("-ln(x)^3 on (0,1) = 6", |x| -(x.ln().powi(3)), 0.0, 1.0),
    ];
    for (label, f, a, b) in cases {
        let r = tanhsinh(f, a, b, 0.0, 1e-12, 16);
        println!(
            "{label} -> {:.15e} err={:.3e} neval={} conv={}",
            r.integral, r.error, r.neval, r.converged
        );
    }
}
