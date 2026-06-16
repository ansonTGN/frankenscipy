use fsci_stats::{ContinuousDistribution, VonMises};
fn main() {
    for &k in &[0.1, 0.5, 1.0, 2.0, 5.0, 10.0, 50.0] {
        let d = VonMises::new(k, 0.0);
        println!(
            "vm,{k},{:.17e},{:.17e},{:.17e},{:.17e}",
            d.var(),
            d.skewness(),
            d.kurtosis(),
            d.entropy()
        );
    }
}
