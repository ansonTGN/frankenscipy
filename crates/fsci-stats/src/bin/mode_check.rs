use fsci_stats::*;
fn check<D: ContinuousDistribution>(name: &str, d: &D) {
    let m = d.mode();
    if !m.is_finite() {
        return;
    }
    let lo = d.ppf(0.0005);
    let hi = d.ppf(0.9995);
    if !(lo.is_finite() && hi.is_finite() && hi > lo) {
        return;
    }
    let n = 400_000usize;
    let mut best_x = lo;
    let mut best_p = d.pdf(lo);
    for i in 0..=n {
        let x = lo + (hi - lo) * (i as f64) / (n as f64);
        let p = d.pdf(x);
        if p > best_p {
            best_p = p;
            best_x = x;
        }
    }
    let pm = d.pdf(m);
    // Flag if the scan finds pdf meaningfully higher than at the claimed mode.
    if best_p > pm * (1.0 + 1e-3) && (best_x - m).abs() > 2.0 * (hi - lo) / (n as f64) {
        println!(
            "{name},MISMATCH,mode={m:.6e},pdf(mode)={pm:.6e},argmax={best_x:.6e},pdf(argmax)={best_p:.6e}"
        );
    } else {
        println!("{name},ok,mode={m:.6e}");
    }
}
fn main() {
    check("norm", &Normal::new(0.0, 1.0));
    check("t5", &StudentT::new(5.0));
    check("chi2_4", &ChiSquared::new(4.0));
    check("chi3", &Chi::new(3.0));
    check("gamma2", &GammaDist::new(2.0, 1.0));
    check("gamma3.5", &GammaDist::new(3.5, 2.0));
    check("beta2_3", &BetaDist::new(2.0, 3.0));
    check("beta3_2", &BetaDist::new(3.0, 2.0));
    check("f5_10", &FDistribution::new(5.0, 10.0));
    check("weibull1.5", &Weibull::new(1.5, 1.0));
    check("weibull3", &Weibull::new(3.0, 2.0));
    check("lognorm1", &Lognormal::new(1.0, 1.0));
    check("pareto3", &Pareto::new(3.0, 1.0));
    check("lomax3", &Lomax::new(3.0));
    check("rayleigh", &Rayleigh::new(1.0));
    check("maxwell", &Maxwell::new(1.0));
    check("gumbel_r", &Gumbel::new(0.0, 1.0));
    check("gumbel_l", &GumbelLeft::new(0.0, 1.0));
    check("logistic", &Logistic::new(0.0, 1.0));
    check("laplace", &Laplace::new(0.0, 1.0));
    check("gompertz", &Gompertz::new(1.5));
    check("nakagami2", &Nakagami::new(2.0));
    check("rice1", &Rice::new(1.0));
    check("genextreme0.2", &GenExtreme::new(0.2));
    check("genpareto0.2", &GenPareto::new(0.2));
    check("invgauss0.5", &InverseGaussian::new(0.5));
    check("invgamma3", &InverseGamma::new(3.0));
    check("fisk3", &Fisk::new(3.0));
    check("skewnorm4", &SkewNorm::new(4.0));
    check("exponnorm1.5", &ExponNorm::new(1.5));
    check("powernorm2", &PowerNorm::new(2.0));
    check("genlogistic2", &GenLogistic::new(2.0));
    check("gennorm1.5", &GenNorm::new(1.5));
    check("foldnorm1.5", &FoldedNormal::new(1.5));
    check("bradford2", &Bradford::new(2.0));
    check("fatiguelife1.5", &FatigueLife::new(1.5));
    check("gengamma2_1.5", &GenGamma::new(2.0, 1.5));
    check("johnsonsu1_2", &JohnsonSU::new(1.0, 2.0));
    check("johnsonsb1_2", &JohnsonSB::new(1.0, 2.0));
    check("pearson3_0.5", &Pearson3::new(0.5));
}
