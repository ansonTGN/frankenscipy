use fsci_stats::*;
fn main() {
    let cases: &[(&str, Vec<f64>, Vec<f64>)] = &[
        (
            "ties",
            vec![1., 1., 2., 2., 3., 3., 4., 4.],
            vec![1., 2., 2., 3., 3., 4., 4., 5.],
        ),
        (
            "xties",
            vec![1., 1., 1., 2., 2., 3., 4., 5.],
            vec![2., 5., 1., 4., 3., 6., 8., 7.],
        ),
        (
            "yties",
            vec![1., 2., 3., 4., 5., 6., 7., 8.],
            vec![1., 1., 2., 2., 3., 3., 4., 4.],
        ),
        (
            "notie_large",
            (0..30).map(|i| (i as f64 * 0.37).sin()).collect(),
            (0..30).map(|i| (i as f64 * 0.29).cos()).collect(),
        ),
        (
            "heavy_ties",
            vec![1., 1., 1., 1., 2., 2., 2., 2., 3., 3.],
            vec![1., 1., 2., 2., 2., 3., 3., 3., 3., 4.],
        ),
    ];
    for (name, x, y) in cases {
        let r = kendalltau(x, y);
        println!("{name},{:.17e},{:.17e}", r.statistic, r.pvalue);
        let rg = kendalltau_alternative(x, y, "greater");
        println!("{name}_greater,{:.17e},{:.17e}", rg.statistic, rg.pvalue);
    }
}
