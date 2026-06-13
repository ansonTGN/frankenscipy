//! discrete moments (mean/var/skew/kurt) probe vs scipy (gitignored).
use fsci_stats::*;
fn emit<D: DiscreteDistribution>(label: &str, d: &D) {
    println!("{label},mean,{:.17e}", d.mean());
    println!("{label},var,{:.17e}", d.var());
    println!("{label},skew,{:.17e}", d.skewness());
    println!("{label},kurt,{:.17e}", d.kurtosis());
}
fn main() {
    emit("poisson|3.5", &Poisson::new(3.5));
    emit("skellam|2_1.3", &Skellam::new(2.0, 1.3));
    emit("binom|20_0.3", &Binomial::new(20, 0.3));
    emit("bernoulli|0.35", &Bernoulli::new(0.35));
    emit("geom|0.25", &Geometric::new(0.25));
    emit("boltzmann|0.7_12", &Boltzmann::new(0.7, 12));
    emit("planck|0.8", &Planck::new(0.8));
    emit("hypergeom|30_10_12", &Hypergeometric::new(30, 10, 12));
    emit("betabinom|20_3_4", &BetaBinomial::new(20, 3.0, 4.0));
    emit("betabinom|15_2.5_5", &BetaBinomial::new(15, 2.5, 5.0));
    emit("dlaplace|0.8", &DiscreteLaplace::new(0.8));
    emit("logser|0.6", &LogSeries::new(0.6));
    emit("nbinom|10_0.4", &NegBinomial::new(10.0, 0.4));
    emit("nhypergeom|30_12_5", &NegHypergeometric::new(30, 12, 5));
}
