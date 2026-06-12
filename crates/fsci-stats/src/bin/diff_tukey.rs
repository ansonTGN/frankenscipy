use fsci_stats::*;
fn main() {
    let g1 = [24.5, 23.5, 26.4, 27.1, 29.9];
    let g2 = [28.4, 34.2, 29.5, 32.2, 30.1];
    let g3 = [26.1, 28.3, 24.3, 26.2, 27.8];
    let r = tukey_hsd(&[&g1, &g2, &g3]);
    for i in 0..3 {
        for j in 0..3 {
            println!("tukey_{i}_{j},{:.17e},0", r.pvalue[i][j]);
        }
    }
}
