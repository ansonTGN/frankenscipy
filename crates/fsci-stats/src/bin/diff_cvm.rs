use fsci_stats::*;
fn main() {
    let cases: [(&[f64], &[f64]); 3] = [
        (&[1., 2., 3., 4., 5.], &[1.5, 2.5, 3.5, 8., 9.]),
        (&[1., 3., 5., 7.], &[2., 4., 6., 8.]),
        (
            &[0.5, 1.2, 2.1, 3.3, 4.4, 5.5],
            &[1.1, 2.2, 6.6, 7.7, 8.8, 9.9],
        ),
    ];
    for (i, (x, y)) in cases.iter().enumerate() {
        let r = cramervonmises_2samp(x, y);
        println!("cvm{i},{:.17e},{:.17e}", r.statistic, r.pvalue);
    }
}
