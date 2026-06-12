use fsci_stats::*;
fn emit<D: ContinuousDistribution>(label: &str, d: &D, xs: &[f64]) {
    for &x in xs {
        println!("{label},logpdf,{x},{:.17e}", d.logpdf(x));
        println!("{label},logsf,{x},{:.17e}", d.logsf(x));
        println!("{label},logcdf,{nx},{:.17e}", d.logcdf(-x), nx = -x);
    }
}
fn main() {
    let far = [10.0, 30.0, 60.0];
    emit("norm|0|1", &Normal::new(0.0, 1.0), &far);
    emit("t|5|0", &StudentT::new(5.0), &[50.0, 300.0]);
    emit("gamma|2|1", &GammaDist::new(2.0, 1.0), &[40.0, 120.0]);
    emit("beta|2|3", &BetaDist::new(2.0, 3.0), &[0.0]); // beta domain (0,1)
    emit("chi2|4|0", &ChiSquared::new(4.0), &[80.0, 150.0]);
    emit("expon|1|0", &Exponential::new(1.0), &[40.0, 200.0]);
    emit("logistic|0|1", &Logistic::new(0.0, 1.0), &[40.0, 200.0]);
    emit("laplace|0|1", &Laplace::new(0.0, 1.0), &[40.0, 200.0]);
    emit("gumbel_r|0|1", &Gumbel::new(0.0, 1.0), &[40.0, 200.0]);
    emit("cauchy|0|1", &Cauchy::new(0.0, 1.0), &[1.0e6, 1.0e10]);
    emit("rayleigh|1|0", &Rayleigh::new(1.0), &[10.0, 30.0]);
    emit("maxwell|1|0", &Maxwell::new(1.0), &[8.0, 15.0]);
    emit("weibull|1.5|1", &Weibull::new(1.5, 1.0), &[25.0, 60.0]);
    emit("lognorm|1|1", &Lognormal::new(1.0, 1.0), &[1.0e4, 1.0e7]);
    emit("f|5|10", &FDistribution::new(5.0, 10.0), &[50.0, 500.0]);
    emit("invgauss|0.5|0", &InverseGaussian::new(0.5), &[15.0, 40.0]);
    emit("gennorm|1.5|0", &GenNorm::new(1.5), &[10.0, 30.0]);
    emit("genlogistic|2|0", &GenLogistic::new(2.0), &[40.0, 200.0]);
    emit("nakagami|2|0", &Nakagami::new(2.0), &[5.0, 10.0]);
    emit("gompertz|1.5|0", &Gompertz::new(1.5), &[5.0, 10.0]);
}
