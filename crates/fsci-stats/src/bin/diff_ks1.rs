use fsci_stats::*;
// Probe ks_1samp two-sided against scipy by testing on a uniform CDF.
fn main() {
    // emit (n, D, pvalue) by constructing data with a known D is hard; instead
    // test the SF directly via a thin re-export is unavailable, so test ks_1samp
    // on uniform[0,1] samples and let python recompute D and scipy SF.
    struct Lcg(u64);
    impl Lcg {
        fn nf(&mut self) -> f64 {
            self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1);
            ((self.0 >> 11) as f64) / ((1u64 << 53) as f64)
        }
    }
    let mut rng = Lcg(99);
    for _ in 0..600 {
        let n = 2 + (rng.0 as usize % 200);
        let mut xs: Vec<f64> = (0..n).map(|_| rng.nf()).collect();
        let r = ks_1samp(&xs, |v| v.clamp(0.0, 1.0)); // uniform CDF
        xs.sort_by(|a, b| a.total_cmp(b));
        let data: Vec<String> = xs.iter().map(|v| format!("{v:.10}")).collect();
        println!(
            "KS1\t{}\t{:.17e}\t{:.17e}",
            data.join(" "),
            r.statistic,
            r.pvalue
        );
    }
}
