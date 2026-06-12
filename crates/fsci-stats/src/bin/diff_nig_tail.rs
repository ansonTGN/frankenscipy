use fsci_stats::*;
fn main() {
    for (a, b) in [(1.0, 0.5), (2.0, -1.0), (1.5, 0.0), (3.0, 2.0), (2.0, 0.7)] {
        let d = NormInvGauss::new(a, b);
        for x in [-2.0, -5.0, -10.0, -20.0, -40.0, -60.0, -100.0, 0.0, 2.0] {
            println!("nig|{a}|{b},cdf,{x},{:.17e}", d.cdf(x));
        }
    }
}
