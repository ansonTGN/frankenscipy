use fsci_stats::*;
fn main() {
    let tables: [[[f64; 2]; 2]; 6] = [
        [[8., 2.], [1., 5.]],
        [[10., 0.], [0., 10.]],
        [[3., 5.], [7., 2.]],
        [[1., 9.], [11., 3.]],
        [[5., 5.], [5., 5.]],
        [[12., 5.], [29., 2.]],
    ];
    for (i, t) in tables.iter().enumerate() {
        let r = fisher_exact(t);
        println!("fisher{i},{:.17e},{:.17e}", r.odds_ratio, r.pvalue);
    }
}
