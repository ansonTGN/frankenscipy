use fsci_stats::*;
// deterministic LCG for reproducible random samples
struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1);
        self.0 >> 11
    }
}
fn main() {
    let mut rng = Lcg(12345);
    for case in 0..400 {
        let m = 2 + (rng.next() % 9) as usize; // 2..10
        let n = 2 + (rng.next() % 11) as usize; // 2..12
        // distinct integer pool to avoid ties (no-tie regime)
        let mut pool: Vec<i64> = (1..=400).collect();
        // shuffle by swaps
        for i in (1..pool.len()).rev() {
            let j = (rng.next() as usize) % (i + 1);
            pool.swap(i, j);
        }
        let x: Vec<f64> = pool[..m].iter().map(|&v| v as f64).collect();
        let y: Vec<f64> = pool[m..m + n].iter().map(|&v| v as f64).collect();
        for alt in ["two-sided", "less", "greater"] {
            let r = mannwhitneyu_alternative(&x, &y, alt);
            // emit args so python can recompute scipy
            let xs: Vec<String> = x.iter().map(|v| format!("{v}")).collect();
            let ys: Vec<String> = y.iter().map(|v| format!("{v}")).collect();
            println!(
                "MWU\t{case}\t{alt}\t{}\t{}\t{:.17e}\t{:.17e}",
                xs.join(" "),
                ys.join(" "),
                r.statistic,
                r.pvalue
            );
        }
    }
}
