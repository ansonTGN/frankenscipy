use fsci_stats::*;
fn p(name: &str, q: f64, v: f64) {
    println!("{name} {q:.3e} {v:.17e}");
}
fn main() {
    let qs = [1e-3, 1e-6, 1e-10, 1e-15, 1e-30, 1e-100];
    macro_rules! sweep {
        ($name:expr, $d:expr) => {{
            let d = $d;
            for &q in &qs {
                p($name, q, d.isf(q));
            }
        }};
    }
    sweep!("norm", Normal::new(0.0, 1.0));
    sweep!("expon", Exponential::new(1.0));
    sweep!("logistic", Logistic::new(0.0, 1.0));
    sweep!("gumbel_r", Gumbel::new(0.0, 1.0));
    sweep!("laplace", Laplace::new(0.0, 1.0));
    sweep!("cauchy", Cauchy::new(0.0, 1.0));
    sweep!("t5", StudentT::new(5.0));
    sweep!("chi2_4", ChiSquared::new(4.0));
    sweep!("gamma2", GammaDist::new(2.0, 1.0));
    sweep!("rayleigh", Rayleigh::new(1.0));
    sweep!("maxwell", Maxwell::new(1.0));
    sweep!("pareto2", Pareto::new(2.0, 1.0));
    sweep!("weibull1.5", Weibull::new(1.5, 1.0));
    sweep!("lognorm", Lognormal::new(0.0, 1.0));
}
