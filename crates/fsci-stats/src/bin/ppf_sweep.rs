use fsci_stats::*;
fn emit<D: ContinuousDistribution>(label: &str, d: &D) {
    for q in [0.05, 0.1, 0.25, 0.5, 0.75, 0.9, 0.95] {
        println!("{label},{q},{:.17e}", d.ppf(q));
    }
}
fn main() {
    emit("norm|0|1", &Normal::new(0.0, 1.0));
    emit("t|5|0", &StudentT::new(5.0));
    emit("chi2|4|0", &ChiSquared::new(4.0));
    emit("chi|3|0", &Chi::new(3.0));
    emit("gamma|2|1", &GammaDist::new(2.0, 1.0));
    emit("gamma|3.5|2", &GammaDist::new(3.5, 2.0));
    emit("beta|2|3", &BetaDist::new(2.0, 3.0));
    emit("beta|0.5|0.5", &BetaDist::new(0.5, 0.5));
    emit("f|5|10", &FDistribution::new(5.0, 10.0));
    emit("weibull_min|1.5|1", &Weibull::new(1.5, 1.0));
    emit("lognorm|1|1", &Lognormal::new(1.0, 1.0));
    emit("pareto|3|1", &Pareto::new(3.0, 1.0));
    emit("lomax|3|0", &Lomax::new(3.0));
    emit("rayleigh|1|0", &Rayleigh::new(1.0));
    emit("maxwell|1|0", &Maxwell::new(1.0));
    emit("gumbel_r|0|1", &Gumbel::new(0.0, 1.0));
    emit("gumbel_l|0|1", &GumbelLeft::new(0.0, 1.0));
    emit("logistic|0|1", &Logistic::new(0.0, 1.0));
    emit("laplace|0|1", &Laplace::new(0.0, 1.0));
    emit("cauchy|0|1", &Cauchy::new(0.0, 1.0));
    emit("expon|1|0", &Exponential::new(1.0));
    emit("gompertz|1.5|0", &Gompertz::new(1.5));
    emit("nakagami|2|0", &Nakagami::new(2.0));
    emit("rice|1|0", &Rice::new(1.0));
    emit("genextreme|-0.2|0", &GenExtreme::new(-0.2));
    emit("genpareto|0.2|0", &GenPareto::new(0.2));
    emit("invgauss|0.5|0", &InverseGaussian::new(0.5));
    emit("invgamma|3|0", &InverseGamma::new(3.0));
    emit("fisk|3|0", &Fisk::new(3.0));
    emit("foldnorm|1.5|0", &FoldedNormal::new(1.5));
    emit("powernorm|2|0", &PowerNorm::new(2.0));
    emit("genlogistic|2|0", &GenLogistic::new(2.0));
    emit("gennorm|1.5|0", &GenNorm::new(1.5));
    emit("bradford|2|0", &Bradford::new(2.0));
    emit("fatiguelife|1.5|0", &FatigueLife::new(1.5));
    emit("dweibull|2|0", &DoubleWeibull::new(2.0));
    emit("gengamma|2|1.5", &GenGamma::new(2.0, 1.5));
    emit("dgamma|2|0", &DoubleGamma::new(2.0));
    emit("halfnorm|0|0", &HalfNormal);
    emit("halflogistic|0|0", &HalfLogistic);
    emit("hypsecant|0|0", &HypSecant);
    emit("semicircular|0|0", &Semicircular);
    emit("anglit|0|0", &Anglit);
    emit("arcsine|0|0", &Arcsine);
    emit("cosine|0|0", &CosineDistribution);
    emit("skewnorm|4|0", &SkewNorm::new(4.0));
    emit("exponnorm|1.5|0", &ExponNorm::new(1.5));
    emit("johnsonsu|1|2", &JohnsonSU::new(1.0, 2.0));
    emit("johnsonsb|1|2", &JohnsonSB::new(1.0, 2.0));
}
