//! discrete moments (mean/var/skew/kurt/entropy) probe vs scipy (gitignored).
use fsci_stats::*;
fn emit<D: DiscreteDistribution>(label: &str, d: &D) {
    println!("{label},mean,{:.17e}", d.mean());
    println!("{label},var,{:.17e}", d.var());
    println!("{label},skew,{:.17e}", d.skewness());
    println!("{label},kurt,{:.17e}", d.kurtosis());
    println!("{label},entropy,{:.17e}", d.entropy());
}
fn main() {
    emit("poisson|3.5", &Poisson::new(3.5));
    emit("skellam|2_1.3", &Skellam::new(2.0, 1.3));
    emit("binom|20_0.3", &Binomial::new(20, 0.3));
    emit("bernoulli|0.35", &Bernoulli::new(0.35));
    emit("geom|0.25", &Geometric::new(0.25));
    emit("boltzmann|0.7_12", &Boltzmann::new(0.7, 12));
    emit("yulesimon|3.5", &YuleSimon::new(3.5));
    emit("planck|0.8", &Planck::new(0.8));
    emit("hypergeom|30_10_12", &Hypergeometric::new(30, 10, 12));
    emit("zipfian|1.5_50", &Zipfian::new(1.5, 50));
}
