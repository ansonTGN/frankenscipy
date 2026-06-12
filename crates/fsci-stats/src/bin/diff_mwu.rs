use fsci_stats::*;
fn main() {
    let cases: [(&[f64], &[f64]); 4] = [
        (&[1., 2., 3., 4.], &[5., 6., 7., 8.]),
        (&[1., 3., 5., 7.], &[2., 4., 6., 8.]),
        (&[2., 4., 6.], &[1., 3., 5., 7., 9.]),
        (&[1., 2., 3., 4., 5., 6.], &[7., 8., 9., 10., 11., 12.]),
    ];
    for (i, (x, y)) in cases.iter().enumerate() {
        for alt in ["two-sided", "less", "greater"] {
            let r = mannwhitneyu_alternative(x, y, alt);
            println!("case{i}_{alt},{:.17e},{:.17e}", r.statistic, r.pvalue);
        }
    }
}
