use fsci_stats::{BetaNegativeBinomial, DiscreteDistribution};
fn main() {
    for &(n, a, b) in &[
        (10u64, 5.0, 4.0),
        (20, 6.0, 3.0),
        (8, 10.0, 2.0),
        (10, 7.0, 4.0),
        (15, 8.0, 5.0),
        (5, 6.0, 2.0),
        (30, 5.5, 3.5),
    ] {
        let d = BetaNegativeBinomial::new(n, a, b);
        println!(
            "bnb,{n},{a},{b},{:.17e},{:.17e},{:.17e},{:.17e}",
            d.mean(),
            d.var(),
            d.skewness(),
            d.kurtosis()
        );
    }
}
