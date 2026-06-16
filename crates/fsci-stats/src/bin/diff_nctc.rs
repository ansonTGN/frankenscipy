use fsci_stats::*;
fn main() {
    let d = NoncentralT::new(10.0, 5.0);
    for &t in &[-1.81, -3.0, -6.0, -12.96] {
        println!("{t},{:.17e}", d.cdf(t));
    }
    let d2 = NoncentralT::new(30.0, 1.0);
    for &t in &[11.45, 13.54, 15.0] {
        println!("r_{t},{:.17e}", d2.cdf(t));
    }
}
