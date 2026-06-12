use fsci_stats::*;
fn main() {
    for &c in &[0.2_f64, -0.3] {
        let d = GenExtreme::new(c);
        println!(
            "# c={c} mean={:.17e} var={:.17e} skew={:.17e} kurt={:.17e}",
            d.mean(),
            d.var(),
            d.skewness(),
            d.kurtosis()
        );
        // dump pdf over a wide grid
        let mut x = -20.0_f64;
        while x <= 60.0 {
            println!("{c},{x},{:.17e},{:.17e}", d.pdf(x), d.cdf(x));
            x += 0.01;
        }
    }
}
