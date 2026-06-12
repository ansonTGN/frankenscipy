use fsci_stats::*;
fn main() {
    let x = [0.1, 0.25, 0.4, 0.55, 0.7, 0.85];
    let r = cramervonmises(&x, |v| v.clamp(0.0, 1.0));
    println!("cvm1samp,{:.17e},{:.17e}", r.statistic, r.pvalue);
    let g1 = vec![1., 2., 3., 4., 5., 6., 7.];
    let g2 = vec![2., 3., 4., 5., 6., 7., 8.];
    let g3 = vec![1.5, 2.5, 8., 9., 10., 11., 12.];
    if let Ok(a) = anderson_ksamp(&[g1, g2, g3], None) {
        println!("anderson_ksamp,{:.17e},{:.17e}", a.statistic, a.pvalue);
    }
}
