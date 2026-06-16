use fsci_stats::{Alpha, ContinuousDistribution};
fn main() {
    for &a in &[1.0, 2.0, 3.0] {
        let d = Alpha::new(a);
        for &x in &[0.5, 1.0, 2.0, 5.0, 20.0, 100.0, 1e4, 1e8] {
            println!("alpha_sf,{a},{x:.3e},{:.17e}", d.sf(x));
            println!("alpha_cdf,{a},{x:.3e},{:.17e}", d.cdf(x));
        }
    }
}
