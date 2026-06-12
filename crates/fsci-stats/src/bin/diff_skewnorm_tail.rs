use fsci_stats::*;
fn main() {
    for a in [0.5, 1.0, 2.0, 4.0, 8.0] {
        let d = SkewNorm::new(a);
        for x in [-3.0, -3.5, -4.0, -5.0, -6.0, -8.0, -10.0, -12.0] {
            println!("skewnorm|{a}|0,cdf,{x},{:.17e}", d.cdf(x));
        }
    }
}
