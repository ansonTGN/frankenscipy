use fsci_stats::*;
fn emit<D: ContinuousDistribution>(label: &str, d: &D, xs: &[f64]) {
    for &x in xs {
        println!("{label},cdf,{x},{:.17e}", d.cdf(x));
        println!("{label},logcdf,{x},{:.17e}", d.logcdf(x));
    }
}
fn main() {
    let n = [-8.0, -15.0, -30.0];
    let f = [-20.0, -60.0, -150.0];
    emit("t|5|0", &StudentT::new(5.0), &[-50.0, -300.0]);
    emit("laplace|0|1", &Laplace::new(0.0, 1.0), &[-40.0, -200.0]);
    emit("logistic|0|1", &Logistic::new(0.0, 1.0), &[-40.0, -200.0]);
    emit("gumbel_l|0|1", &GumbelLeft::new(0.0, 1.0), &n);
    emit("cauchy|0|1", &Cauchy::new(0.0, 1.0), &[-1.0e6, -1.0e10]);
    emit("gennorm|2|0", &GenNorm::new(2.0), &n);
    emit("gennorm|1|0", &GenNorm::new(1.0), &f);
    emit("genlogistic|2|0", &GenLogistic::new(2.0), &[-40.0, -200.0]);
    emit("hypsecant|0|0", &HypSecant, &[-40.0, -200.0]);
    emit("exponnorm|1.5|0", &ExponNorm::new(1.5), &[-10.0, -30.0]);
    emit("skewnorm|4|0", &SkewNorm::new(4.0), &[-8.0, -15.0]);
    emit("dgamma|2|0", &DoubleGamma::new(2.0), &[-30.0, -80.0]);
    emit("dweibull|2|0", &DoubleWeibull::new(2.0), &[-6.0, -12.0]);
    emit(
        "johnsonsu|1|2",
        &JohnsonSU::new(1.0, 2.0),
        &[-1.0e3, -1.0e6],
    );
    emit(
        "norminvgauss|1|0.5",
        &NormInvGauss::new(1.0, 0.5),
        &[-20.0, -60.0],
    );
    emit("powernorm|2|0", &PowerNorm::new(2.0), &n);
}
