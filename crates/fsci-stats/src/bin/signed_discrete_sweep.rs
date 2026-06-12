use fsci_stats::*;
fn main() {
    // Skellam: pmf_signed over signed support + moments
    let skellams = [(3.0_f64, 2.0_f64), (1.0, 4.0), (5.0, 5.0), (0.7, 0.3)];
    for &(mu1, mu2) in &skellams {
        let d = Skellam::new(mu1, mu2);
        let lab = format!("skellam|{mu1}|{mu2}");
        println!("{lab},mean,{:.17e}", d.mean());
        println!("{lab},var,{:.17e}", d.var());
        println!("{lab},skew,{:.17e}", d.skewness());
        println!("{lab},kurt,{:.17e}", d.kurtosis());
        for k in -10i64..=10 {
            println!("{lab},pmf{k},{:.17e}", d.pmf_signed(k));
        }
    }
    // DiscreteLaplace: pmf_signed + moments + entropy
    for a in [0.5_f64, 1.0, 1.5, 2.5] {
        let d = DiscreteLaplace::new(a);
        let lab = format!("dlaplace|{a}|0");
        println!("{lab},mean,{:.17e}", d.mean());
        println!("{lab},var,{:.17e}", d.var());
        println!("{lab},skew,{:.17e}", d.skewness());
        println!("{lab},kurt,{:.17e}", d.kurtosis());
        println!("{lab},entropy,{:.17e}", d.entropy());
        for k in -10i64..=10 {
            println!("{lab},pmf{k},{:.17e}", d.pmf_signed(k));
        }
    }
}
