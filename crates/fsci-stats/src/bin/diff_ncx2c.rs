use fsci_stats::*;
fn main() {
    for &b in &[0.5, 2.0, 5.0, 20.0, 40.0] {
        let d = Rice::new(b);
        for &x in &[0.3, b, b + 1.0, b + 5.0] {
            println!("{b},{x},{:.16e}", d.cdf(x));
        }
    }
}
