use fsci_stats::*;
fn t2(name: &str, s: f64, p: f64) {
    println!("{name},{s:.17e},{p:.17e}");
}
fn main() {
    let a = [5.1, 4.9, 6.2, 5.8, 5.5, 6.0, 5.3, 4.7, 6.1, 5.9];
    let b = [6.0, 5.8, 7.1, 6.5, 6.3, 7.0, 6.2, 5.6, 6.8, 6.7];
    let c = [2.0, 3.0, 5.0, 7.0, 11.0, 13.0, 17.0, 19.0, 23.0, 29.0];
    // ttests
    let r = ttest_ind(&a, &b);
    t2("ttest_ind", r.statistic, r.pvalue);
    let r = ttest_rel(&a, &b, None).unwrap();
    t2("ttest_rel", r.statistic, r.pvalue);
    let r = ttest_1samp(&a, 5.0);
    t2("ttest_1samp", r.statistic, r.pvalue);
    // normality tests
    let r = shapiro(&c);
    t2("shapiro", r.statistic, r.pvalue);
    let r = normaltest(&c);
    t2("normaltest", r.statistic, r.pvalue);
    let r = jarque_bera(&c);
    t2("jarque_bera", r.statistic, r.pvalue);
    let r = skewtest(&c, None, Some("two-sided")).unwrap();
    t2("skewtest", r.statistic, r.pvalue);
    let r = kurtosistest(&c, None, Some("two-sided")).unwrap();
    t2("kurtosistest", r.statistic, r.pvalue);
    // fisher / chi2
    let r = fisher_exact(&[[8.0, 2.0], [1.0, 5.0]]);
    t2("fisher_exact", r.odds_ratio, r.pvalue);
    let r = chi2_contingency(&[vec![10.0, 20.0, 30.0], vec![6.0, 9.0, 17.0]], true);
    t2("chi2_contingency", r.statistic, r.pvalue);
    // robust slopes
    let r = theilslopes(&c, &a, 0.95);
    t2("theilslopes_slope", r.slope, r.intercept);
    let r = siegelslopes(&c, &a);
    t2("siegelslopes", r.slope, r.intercept);
    let bin = [0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0, 1.0, 1.0];
    let r = pointbiserialr(&bin, &c);
    t2("pointbiserialr", r.statistic, r.pvalue);
}
