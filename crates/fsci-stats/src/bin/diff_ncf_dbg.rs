use fsci_stats::*;
fn main() {
    for (d1, d2, nc) in [(5.0, 12.0, 3.0), (4.0, 20.0, 2.0), (8.0, 8.0, 5.0)] {
        let d = NoncentralF::new(d1, d2, nc);
        for x in [0.5, 1.0, 1.92, 3.0, 6.0, 20.0] {
            println!("pdf,{d1},{d2},{nc},{x},{:.15e}", d.pdf(x));
            println!("cdf,{d1},{d2},{nc},{x},{:.15e}", d.cdf(x));
        }
        println!("entropy,{d1},{d2},{nc},0,{:.15e}", d.entropy());
        println!("skew,{d1},{d2},{nc},0,{:.15e}", d.skewness());
        println!("kurt,{d1},{d2},{nc},0,{:.15e}", d.kurtosis());
    }
}
