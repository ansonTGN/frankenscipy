//! ppf/isf tail-quantile sweep vs scipy (gitignored).
use fsci_stats::*;
fn emit<D: ContinuousDistribution>(label: &str, d: &D, qs: &[f64]) {
    for &q in qs {
        println!("{label},ppf,{q:.17e},{:.17e}", d.ppf(q));
        println!("{label},isf,{q:.17e},{:.17e}", d.isf(q));
    }
}
fn main() {
    let qs = [
        1.0e-12,
        1.0e-8,
        1.0e-4,
        1.0e-2,
        0.1,
        0.25,
        0.5,
        0.75,
        0.9,
        0.99,
        1.0 - 1.0e-4,
        1.0 - 1.0e-8,
    ];
    emit("norm|0|1", &Normal::new(0.0, 1.0), &qs);
    emit("t|5|0", &StudentT::new(5.0), &qs);
    emit("gamma|2|1", &GammaDist::new(2.0, 1.0), &qs);
    emit("beta|2|3", &BetaDist::new(2.0, 3.0), &qs);
    emit("chi2|4|0", &ChiSquared::new(4.0), &qs);
    emit("expon|1|0", &Exponential::new(1.0), &qs);
    emit("logistic|0|1", &Logistic::new(0.0, 1.0), &qs);
    emit("laplace|0|1", &Laplace::new(0.0, 1.0), &qs);
    emit("gumbel_r|0|1", &Gumbel::new(0.0, 1.0), &qs);
    emit("cauchy|0|1", &Cauchy::new(0.0, 1.0), &qs);
    emit("rayleigh|1|0", &Rayleigh::new(1.0), &qs);
    emit("maxwell|1|0", &Maxwell::new(1.0), &qs);
    emit("weibull_min|1.5|1", &Weibull::new(1.5, 1.0), &qs);
    emit("lognorm|1|1", &Lognormal::new(1.0, 1.0), &qs);
    emit("f|5|10", &FDistribution::new(5.0, 10.0), &qs);
    emit("invgauss|0.5|0", &InverseGaussian::new(0.5), &qs);
    emit("gennorm|1.5|0", &GenNorm::new(1.5), &qs);
    emit("genlogistic|2|0", &GenLogistic::new(2.0), &qs);
    emit("nakagami|2|0", &Nakagami::new(2.0), &qs);
    emit("gompertz|1.5|0", &Gompertz::new(1.5), &qs);
}
