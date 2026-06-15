//! binned_statistic_dd probe vs scipy.stats.binned_statistic_dd.
//! Lines: `stat,flatidx,value`. Inputs must match the python comparator.
use fsci_stats::binned_statistic_dd;

fn main() {
    // 20 deterministic 3-D points (a small LCG keyed to index), values = i.
    let n = 20usize;
    let mut sample: Vec<Vec<f64>> = Vec::with_capacity(n);
    let mut values: Vec<f64> = Vec::with_capacity(n);
    for i in 0..n {
        let a = ((i * 1103515245 + 12345) % 1000) as f64 / 1000.0;
        let b = ((i * 1664525 + 1013904223) % 1000) as f64 / 1000.0;
        let c = ((i * 22695477 + 1) % 1000) as f64 / 1000.0;
        sample.push(vec![a, b, c]);
        values.push(i as f64 + 0.5);
    }
    for stat in ["count", "sum", "mean", "min", "max", "median", "std"] {
        let (stats, _edges) = binned_statistic_dd(&sample, &values, 3, stat);
        for (idx, &v) in stats.iter().enumerate() {
            println!("{stat},{idx},{v:.17e}");
        }
    }
}
