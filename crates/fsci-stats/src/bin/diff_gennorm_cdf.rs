use fsci_stats::*;
fn main() {
    let d = GenNorm::new(1.5);
    for x in [-10.0, -8.0, -6.0, -4.0, -30.0, -2.0] {
        println!("gennorm cdf {x} {:.17e}", d.cdf(x));
        println!("gennorm sf {x} {:.17e}", d.sf(x));
    }
}
