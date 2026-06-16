use fsci_stats::*;
fn main() {
    for &x in &[0.5, 1.0, 5.0, 20.0, 41.5, 80.0] {
        println!("maxwell,{x},{:.17e}", Maxwell::new(1.0).logpdf(x));
    }
    for &x in &[-5.0, 0.0, 2.0, 10.0, 39.0, 60.0] {
        println!("skewnorm,{x},{:.17e}", SkewNorm::new(4.0).logpdf(x));
    }
    for &x in &[0.1, 0.5, 2.0, 50.0, 371.0, 800.0] {
        println!("invgauss,{x},{:.17e}", InverseGaussian::new(0.5).logpdf(x));
    }
}
